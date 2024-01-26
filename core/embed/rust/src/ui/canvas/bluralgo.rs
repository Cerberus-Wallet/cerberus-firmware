/// This is a simple and fast blurring algorithm that uses a box filter -
/// a square kernel with all coefficients set to 1.
///
/// The `BlurFilter` structure holds the context of a simple 2D window averaging
/// filter - a sliding window and the sum of all rows in the sliding window.
///
/// The `BlurFilter` implements only three public functions - `new`, `push`, and
/// `pop`.
///
/// The `new()` function creates a blur filter context.
///   - The `width` argument specifies the width of the blurred area.
///   - The `radius` argument specifies the length of the kernel side.
///
/// ```rust
/// let blur = BlurFilter::new(width, radius);
/// ```
///
/// The `push()` function pushes source row data into the sliding window and
/// performs all necessary calculations.
///
/// ```rust
/// blur.push(&canvas.row(ya)[x0..x1]);
/// ```
///
/// The `pop()` function pops the blurred row from the sliding window.
///
/// ```rust
/// blur.pop(&mut canvas.row(yb)[x0..x1]);
/// ```
use crate::trezorhal::gdc::GdcBuffer;

const MAX_RADIUS: usize = 4;
const MAX_SIDE: usize = 1 + MAX_RADIUS * 2;
const MAX_WIDTH: usize = 240;

type PixelColor = u16;

#[derive(Default, Copy, Clone)]
struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl Rgb<u16> {
    #[inline(always)]
    fn mulshift(&self, multiplier: u32, shift: u8) -> Rgb<u8> {
        Rgb::<u8> {
            r: ((self.r as u32 * multiplier) >> shift) as u8,
            g: ((self.g as u32 * multiplier) >> shift) as u8,
            b: ((self.b as u32 * multiplier) >> shift) as u8,
        }
    }
}

impl From<u16> for Rgb<u16> {
    #[inline(always)]
    fn from(value: u16) -> Rgb<u16> {
        Rgb::<u16> {
            r: (value >> 8) & 0xF8,
            g: (value >> 3) & 0xFC,
            b: (value << 3) & 0xF8,
        }
    }
}

impl core::ops::AddAssign<u16> for Rgb<u16> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: u16) {
        let rgb: Rgb<u16> = rhs.into();
        *self += rgb;
    }
}

impl core::ops::SubAssign<u16> for Rgb<u16> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: u16) {
        let rgb: Rgb<u16> = rhs.into();
        *self -= rgb;
    }
}

impl core::ops::AddAssign for Rgb<u16> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl core::ops::SubAssign for Rgb<u16> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Rgb<u16>) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl From<Rgb<u8>> for u16 {
    #[inline(always)]
    fn from(value: Rgb<u8>) -> u16 {
        let r = (value.r as u16 & 0xF8) << 8;
        let g = (value.g as u16 & 0xFC) << 3;
        let b = (value.b as u16 & 0xF8) >> 3;
        r | g | b
    }
}

impl From<Rgb<u16>> for Rgb<u8> {
    #[inline(always)]
    fn from(value: Rgb<u16>) -> Rgb<u8> {
        Rgb::<u8> {
            r: value.r as u8,
            g: value.g as u8,
            b: value.b as u8,
        }
    }
}

impl core::ops::AddAssign<Rgb<u8>> for Rgb<u16> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Rgb<u8>) {
        self.r += rhs.r as u16;
        self.g += rhs.g as u16;
        self.b += rhs.b as u16;
    }
}

impl core::ops::SubAssign<Rgb<u8>> for Rgb<u16> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Rgb<u8>) {
        self.r -= rhs.r as u16;
        self.g -= rhs.g as u16;
        self.b -= rhs.b as u16;
    }
}

#[link_section = ".no_dma_buffers"]
static mut WINDOW: [Rgb<u8>; MAX_WIDTH * MAX_SIDE] =
    [Rgb::<u8> { r: 0, g: 0, b: 0 }; MAX_WIDTH * MAX_SIDE];

fn alloc_window(size: usize) -> &'static mut [Rgb<u8>] {
    unsafe {
        let result = &mut WINDOW[..size];
        result.iter_mut().for_each(|it| {
            *it = Rgb::<u8>::default();
        });
        result
    }
}

pub struct BlurAlgorithm<'a> {
    width: usize,
    radius: usize,
    row: usize,

    //window: GdcBuffer::<'a, RGB>,
    totals: GdcBuffer<'a, Rgb<u16>>,

    window: &'a mut [Rgb<u8>],
    // totals: [RGB; MAX_WIDTH],
}

impl<'a> BlurAlgorithm<'a> {
    /// Constraints:
    ///   width <= MAX_WIDTH
    ///   radius <= MAX_RADIUS
    ///   width >= radius
    pub fn new(width: usize, radius: usize) -> Option<Self> {
        assert!(width <= MAX_WIDTH);
        assert!(radius <= MAX_RADIUS);
        assert!(width > 2 * radius - 1);

        let side = 1 + radius * 2;

        // let window = GdcBuffer::<RGB>::alloc(width * side)?;
        // self.window.iter_mut().for_each(|it| {*it = RGB::default()});

        let window = alloc_window(width * side);

        let totals = GdcBuffer::<Rgb<u16>>::alloc(width)?;
        totals
            .data
            .iter_mut()
            .for_each(|it| *it = Rgb::<u16>::default());

        Some(Self {
            width,
            radius,
            row: 0,
            window,
            totals,
            // window: [RGB::default(); MAX_WIDTH * MAX_SIDE],
            // totals: [RGB::default(); MAX_WIDTH],
        })
    }

    // Returns the length of the box filter side
    fn box_side(&self) -> usize {
        1 + self.radius * 2
    }

    /// Takes an input row and calculates the same-sized vector
    /// as the floating average of n subsequent elements where n = 2 * radius + 1.
    /// Finally, it stores it into the specifed row in the  sliding window.
    fn average_to_row(&mut self, inp: &[PixelColor], row: usize) {
        let radius = self.radius;
        let offset = self.width * row;
        let row = &mut self.window[offset..offset + self.width];

        let mut sum = Rgb::<u16>::default();

        let divisor = (radius * 2 + 1) as u16;
        let shift = 10;
        let multiplier = (1 << shift) as u32 / divisor as u32;

        // Prepare before averaging
        for i in 0..radius {
            sum += inp[0]; // Duplicate pixels on the left
            sum += inp[i]; // Add first radius pixels
        }

        // Process the first few pixels of the row
        for i in 0..radius {
            sum += inp[i + radius];
            row[i] = sum.mulshift(multiplier, shift);
            sum -= inp[0];
        }

        // Process the inner part of the row
        for i in radius..row.len() - radius {
            sum += inp[i + radius];
            row[i] = sum.mulshift(multiplier, shift);
            sum -= inp[i - radius];
        }

        // Process the last few pixels of the row
        for i in (row.len() - radius)..row.len() {
            sum += inp[inp.len() - 1];
            row[i] = sum.mulshift(multiplier, shift);
            sum -= inp[i - radius]; // Duplicate pixels on the right
        }
    }

    /// Subtracts the specified row of sliding window from totals[]
    fn subtract_row(&mut self, row: usize) {
        let offset = row * self.width;
        let row = &self.window[offset..offset + self.width];

        for i in 0..row.len() {
            self.totals.data[i] -= row[i];
        }
    }

    /// Adds the specified row of sliding window to totals[]
    fn add_row(&mut self, row: usize) {
        let offset = row * self.width;
        let row = &self.window[offset..offset + self.width];

        for i in 0..row.len() {
            self.totals.data[i] += row[i];
        }
    }

    /// Takes the source row and pushes it into the sliding window
    pub fn push(&mut self, input: &[PixelColor]) {
        let row = self.row;

        self.subtract_row(row);
        self.average_to_row(input, row);
        self.add_row(row);

        self.row = (row + 1) % self.box_side();
    }

    /// Copies the current content of totals[] to the output buffer
    pub fn pop(&self, output: &mut [PixelColor]) {
        let divisor = self.box_side() as u16;
        let shift = 10;
        let multiplier = (1 << shift) as u32 / divisor as u32;

        for i in 0..output.len() {
            output[i] = self.totals.data[i].mulshift(multiplier, shift).into();
        }
    }
}
