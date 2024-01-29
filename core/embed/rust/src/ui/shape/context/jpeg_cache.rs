use crate::ui::{
    canvas::rgb::BitmapRef,
    display::tjpgd,
    geometry::{Offset, Point, Rect},
};

use crate::trezorhal::gdc::{GdcBitmap, GdcBitmapRef};
use core::cell::UnsafeCell;
use trezor_tjpgdec::JpegOutput;
use without_alloc::{alloc::LocalAllocLeakExt, FixedVec};

// JDEC work buffer size
//
// number of quantization tables (n_qtbl) = 2..4 (typical 2)
// number of huffman tables (n_htbl) = 2..4 (typical 2)
// mcu size = 1 * 1 .. 2 * 2 = 1..4 (typical 4)
//
// hufflut_ac & hufflut_dc are required only if JD_FASTDECODE == 2 (default)
//
// ---------------------------------------------------------------------
// table       | size calculation                   |  MIN..MAX   |  TYP
// ---------------------------------------------------------------------
// qttbl       | n_qtbl * size_of(i32) * 64         |  512..1024  |  512
// huffbits    | n_htbl * size_of(u8) * 16          |   32..64    |   32
// huffcode    | n_htbl * size_of(u16) * 256        | 1024..2048  | 1024
// huffdata    | n_htbl * size_of(u8) * 256         |  512..1024  |  512
// hufflut_ac  | n_htbl * size_of(u16) * 1024       | 4096..8192  | 4096
// hufflut_dc  | n_htbl * size_of(u8) * 1024        | 2048..4096  | 2048
// workbuf     | mcu_size * 192 + 64                |  256..832   |  832
// mcubuf      | (mcu_size + 2) * size_of(u16) * 64 |  384..768   |  768
// inbuff      | JD_SZBUF constant                  |  512..512   |  512
// ---------------------------------------------------------------|------
// SUM         |                                    | 9376..18560 | 10336
// ---------------------------------------------------------------|------

const JPEG_DECODER_POOL_SIZE: usize = 10500; // the same const > 10336 as in original code

pub struct JpegCacheSlot<'a> {
    // Reference to compressed data
    jpeg: &'a [u8],
    // Input buffer referencing compressed data
    input: Option<tjpgd::BufferInput<'a>>,
    // JPEG decoder instance
    decoder: Option<tjpgd::JDEC<'a>>,
    // Scratchpad memory used by the JPEG decoder
    // (it's used just by our decoder and nobody else)
    scratchpad: &'a UnsafeCell<[u8; JPEG_DECODER_POOL_SIZE]>,
}

impl<'a> JpegCacheSlot<'a> {
    fn new<'alloc: 'a, T>(pool: &'alloc T) -> JpegCacheSlot<'a>
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        let scratchpad = unwrap!(pool.alloc_t::<UnsafeCell<[u8; JPEG_DECODER_POOL_SIZE]>>())
            .uninit
            .init(UnsafeCell::new([0; JPEG_DECODER_POOL_SIZE]));

        Self {
            jpeg: &[],
            input: None,
            decoder: None,
            scratchpad: scratchpad,
        }
    }

    fn reset<'i: 'a>(&mut self, jpeg: &'i [u8]) -> Result<(), tjpgd::Error> {
        // Drop the existing decoder holding
        // a mutable reference to the scratchpad buffer
        self.decoder = None;

        if jpeg.len() > 0 {
            // Now there's nobody else holding any reference to our window
            // so we can get a mutable reference and pass it to a new
            // instance of the JPEG decoder
            let scratchpad = unsafe { &mut *self.scratchpad.get() };
            // Prepare a input buffer
            let mut input = tjpgd::BufferInput(jpeg);
            // Initialize the decoder by reading headers from input
            self.decoder = Some(tjpgd::JDEC::new(&mut input, scratchpad)?);
            // Save modified input buffer
            self.input = Some(input);
        } else {
            self.input = None;
        }

        self.jpeg = jpeg;
        Ok(())
    }

    fn is_for<'i: 'a>(&self, jpeg: &'i [u8]) -> bool {
        jpeg == self.jpeg && self.decoder.is_some()
    }

    pub fn get_size<'i: 'a>(&mut self, jpeg: &'i [u8]) -> Result<Offset, tjpgd::Error> {
        if !self.is_for(jpeg) {
            self.reset(jpeg)?;
        }
        let decoder = unwrap!(self.decoder.as_mut());
        Ok(Offset::new(decoder.width(), decoder.height()))
    }

    // left-top origin of output rectangle must be aligned to JPEG MCU size
    pub fn decompress<'i: 'a>(
        &mut self,
        jpeg: &'i [u8],
        offset: Point,
        output: &mut dyn JpegOutput,
    ) -> Result<(), tjpgd::Error> {
        // Reset the slot if the JPEG image is different
        if !self.is_for(jpeg) {
            self.reset(jpeg)?;
        }

        // Get coordinates of the next coming MCU
        let decoder = unwrap!(self.decoder.as_ref());
        let next_mcu = Offset::new(decoder.next_mcu().0 as i16, decoder.next_mcu().1 as i16);

        // Get height of the MCUs (8 or 16pixels)
        let mcu_height = decoder.mcu_height() as i16;

        // Reset the decoder if any part of src_clip was already decoded
        if offset.y < next_mcu.y || (offset.x < next_mcu.x && offset.y < next_mcu.y + mcu_height) {
            self.reset(self.jpeg)?;
        }

        let decoder = unwrap!(self.decoder.as_mut());
        let input = unwrap!(self.input.as_mut());
        match decoder.decomp2(input, output) {
            Ok(_) | Err(tjpgd::Error::Interrupted) => Ok(()),
            Err(e) => return Err(e),
        }
    }
}

pub struct JpegFnOutput<F>
where
    F: FnMut(Rect, &BitmapRef) -> bool,
{
    output: F,
}

impl<F> JpegFnOutput<F>
where
    F: FnMut(Rect, &BitmapRef) -> bool,
{
    pub fn new(output: F) -> Self {
        Self { output }
    }
}

impl<F> trezor_tjpgdec::JpegOutput for JpegFnOutput<F>
where
    F: FnMut(Rect, &BitmapRef) -> bool,
{
    fn write(
        &mut self,
        _jd: &tjpgd::JDEC,
        rect_origin: (u32, u32),
        rect_size: (u32, u32),
        pixels: &[u16],
    ) -> bool {
        // MCU coordinates in source image
        let mcu_r = Rect::from_top_left_and_size(
            Point::new(rect_origin.0 as i16, rect_origin.1 as i16),
            Offset::new(rect_size.0 as i16, rect_size.1 as i16),
        );

        // Create readonly bitmap in the memory not accessible with DMA
        let mcu_bitmap =
            GdcBitmap::from_rgb565_slice(pixels, (mcu_r.width() * 2) as usize, mcu_r.size());

        // Return true to continue decompression
        (self.output)(mcu_r, &GdcBitmapRef::new(&mcu_bitmap))
    }
}

pub struct JpegCache<'a> {
    slots: FixedVec<'a, JpegCacheSlot<'a>>,
}

impl<'a> JpegCache<'a> {
    pub fn new<'alloc: 'a, T>(pool: &'alloc T, slot_count: usize) -> Self
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        assert!(slot_count <= 1); // we support just 1 decoder

        let mut cache = JpegCache {
            slots: unwrap!(pool.fixed_vec(slot_count)),
        };

        for _ in 0..cache.slots.capacity() {
            unwrap!(cache.slots.push(JpegCacheSlot::new(pool)));
        }

        cache
    }

    pub fn get_size<'i: 'a>(&mut self, jpeg: &'i [u8]) -> Result<Offset, tjpgd::Error> {
        if self.slots.capacity() > 0 {
            self.slots[0].get_size(jpeg)
        } else {
            Err(tjpgd::Error::MemoryPool)
        }
    }

    pub fn decompress<'i: 'a>(
        &mut self,
        jpeg: &'i [u8],
        offset: Point,
        output: &mut dyn JpegOutput,
    ) -> Result<(), tjpgd::Error> {
        if self.slots.capacity() > 0 {
            self.slots[0].decompress(jpeg, offset, output)
        } else {
            Err(tjpgd::Error::MemoryPool)
        }
    }
}
