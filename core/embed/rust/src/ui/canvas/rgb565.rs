use crate::ui::{
    canvas::{bluralgo::BlurAlgorithm, rgb, Viewport},
    display::Color,
    geometry::{Offset, Point, Rect},
};

use crate::trezorhal::gdc::{Gdc, GdcBitmap, GdcBuffer};

// ==========================================================================
// RGB565 Canvas
// ==========================================================================

pub struct Canvas<'a> {
    buff: GdcBuffer<'a, u16>,
    bitmap: GdcBitmap<'a>,
    viewport: Viewport,
}

impl<'a> Canvas<'a> {
    pub fn new(size: Offset) -> Option<Self> {
        if size.x > 0 && size.y > 0 {
            let width = size.x as usize;
            let height = size.y as usize;
            let mut buff = GdcBuffer::<u16>::alloc(height * width)?;
            let stride = width * 2;
            let bitmap = GdcBitmap::new_rgb565(&mut buff, stride, size);
            Some(Self {
                buff,
                bitmap,
                viewport: Viewport::from_size(size),
            })
        } else {
            panic!();
        }
    }

    pub fn row(&mut self, row: i16) -> Option<&mut [u16]> {
        if row >= 0 && row < self.bitmap.height() {
            let offset = (self.bitmap.stride()) / 2 * row as usize;
            Some(&mut self.buff.data[offset..offset + self.bitmap.width() as usize])
        } else {
            None
        }
    }

    pub fn row_bytes(&mut self, row: i16, height: i16) -> Option<&mut [u8]> {
        if row >= 0
            && height > 0
            && row < self.bitmap.height()
            && row + height <= self.bitmap.height()
        {
            let offset = self.bitmap.stride() * row as usize;
            let len = self.bitmap.stride() * height as usize;

            unsafe {
                Some(core::slice::from_raw_parts_mut(
                    (self.buff.data.as_mut_ptr() as *mut u8).add(offset),
                    len,
                ))
            }
        } else {
            None
        }
    }

    fn gdc<'b>(&'b mut self) -> Gdc<'b> {
        self.bitmap.gdc()
    }
}

impl<'a> rgb::RgbCanvas for Canvas<'a> {
    fn viewport(&self) -> Viewport {
        self.viewport
    }

    fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport.absolute_clip(self.bounds());
    }

    fn size(&self) -> Offset {
        self.bitmap.size()
    }

    fn fill_rect(&mut self, r: Rect, color: Color) {
        let r = r
            .translate(self.viewport.origin)
            .intersect(self.viewport.clip);
        self.gdc().fill_rect(r, color);
    }

    fn draw_bitmap(&mut self, r: Rect, bitmap: &rgb::BitmapRef) {
        let r_moved = r.translate(self.viewport.origin);
        let r_clipped = r_moved.intersect(self.viewport.clip);
        let new_b = bitmap
            .clone()
            .with_offset(r_clipped.top_left() - r_moved.top_left());
        self.gdc().draw_bitmap(r_clipped, &new_b);
    }

    fn draw_blended(&mut self, r: Rect, fg: &rgb::BitmapRef, bg: &rgb::BitmapRef) {
        let r_moved = r.translate(self.viewport.origin);
        let r_clipped = r_moved.intersect(self.viewport.clip);
        let new_fg = fg
            .clone()
            .with_offset(r_clipped.top_left() - r_moved.top_left());
        let new_bg = bg
            .clone()
            .with_offset(r_clipped.top_left() - r_moved.top_left());
        self.gdc().draw_blended(r_clipped, &new_fg, &new_bg);
    }

    fn draw_opaque_text(&mut self, r: Rect, text: &str, attr: &rgb::TextAttr) {
        let r_moved = r.translate(self.viewport.origin);
        let r_clipped = r_moved.intersect(self.viewport.clip);
        let new_attr = attr
            .clone()
            .with_offset(r_clipped.top_left() - r_moved.top_left());
        self.gdc().draw_opaque_text(r_clipped, text, &new_attr);
    }

    fn draw_blended_text(
        &mut self,
        r: Rect,
        text: &str,
        attr: &rgb::TextAttr,
        bg: &rgb::BitmapRef,
    ) {
        let r_moved = r.translate(self.viewport.origin);
        let r_clipped = r_moved.intersect(self.viewport.clip);
        let new_attr = attr
            .clone()
            .with_offset(r_clipped.top_left() - r_moved.top_left());
        let new_bg = bg
            .clone()
            .with_offset(r_clipped.top_left() - r_moved.top_left());
        self.gdc()
            .draw_blended_text(r_clipped, text, &new_attr, &new_bg);
    }

    fn draw_pixel(&mut self, pt: Point, color: Color) {
        let pt = pt + self.viewport.origin;
        if self.viewport.clip.contains(pt) {
            self.row(pt.y).unwrap()[pt.x as usize] = color.into();
        }
    }
}

impl<'a> rgb::RgbCanvasEx for Canvas<'a> {
    fn bitmap<'b>(&'b self) -> rgb::BitmapRef<'b> {
        rgb::BitmapRef::new(&self.bitmap)
    }

    fn blend_pixel(&mut self, pt: Point, color: Color, alpha: u8) {
        let pt = pt + self.viewport.origin;
        if self.viewport.clip.contains(pt) {
            let bg_color: Color = self.row(pt.y).unwrap()[pt.x as usize].into();
            self.row(pt.y).unwrap()[pt.x as usize] = bg_color.blend(color, alpha).into();
        }
    }

    /// This function applies a blur effect to the specified rectangle.
    //
    /// The blur effect works properly only when the rectangle is not clipped,
    /// which is a strong constraint that's hard to be met. The function uses a
    /// simple box filter, where the 'radius' argument represents the length
    /// of the sides of this filter.
    ///
    /// It's important to be aware that strong artifacts may appear on images
    /// with horizontal/vertical lines
    fn blur_rect(&mut self, r: Rect, radius: usize) {
        let clip = r
            .translate(self.viewport.origin)
            .intersect(self.viewport.clip);

        let ofs = radius as i16;

        if clip.width() > 2 * ofs - 1 && clip.height() > 2 * ofs - 1 {
            let mut blur = BlurAlgorithm::new(clip.width() as usize, radius).unwrap();

            for y in (clip.y0 - ofs)..(clip.y0 + ofs) {
                let rd_y = core::cmp::max(y, clip.y0);
                let row = self.row(rd_y).unwrap();
                blur.push(&row[clip.x0 as usize..clip.x1 as usize]);
            }

            for y in clip.y0..clip.y1 {
                let rd_y = core::cmp::min(y + ofs, clip.y1 - 1);
                let row = self.row(rd_y).unwrap();
                blur.push(&row[clip.x0 as usize..clip.x1 as usize]);

                let row = self.row(y).unwrap();
                blur.pop(&mut row[clip.x0 as usize..clip.x1 as usize]);
            }
        }
    }
}

impl Color {
    pub fn blend(self, fg: Color, alpha: u8) -> Color {
        let alpha = alpha as u16;

        let fg_r: u16 = (fg.to_u16() & 0xF800) >> 11;
        let bg_r: u16 = (self.to_u16() & 0xF800) >> 11;

        let r = (fg_r * alpha + (bg_r * (255 - alpha))) / 255;

        let fg_g: u16 = (fg.to_u16() & 0x07E0) >> 5;
        let bg_g: u16 = (self.to_u16() & 0x07E0) >> 5;
        let g = (fg_g * alpha + (bg_g * (255 - alpha))) / 255;

        let fg_b: u16 = (fg.to_u16() & 0x001F) >> 0;
        let bg_b: u16 = (self.to_u16() & 0x001F) >> 0;
        let b = (fg_b * alpha + (bg_b * (255 - alpha))) / 255;

        return ((r << 11) | (g << 5) | b).into();
    }
}
