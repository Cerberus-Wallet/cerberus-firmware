use crate::ui::{
    canvas::{rgb, Viewport},
    display::Color,
    geometry::{Offset, Point, Rect},
};

use crate::trezorhal::gdc::{Gdc, GdcBitmap, GdcBuffer};

// ==========================================================================
// RGB565 Canvas
// ==========================================================================

pub struct Canvas<'a> {
    buff: GdcBuffer<'a, u32>,
    bitmap: GdcBitmap<'a>,
    viewport: Viewport,
}

impl<'a> Canvas<'a> {
    pub fn new(size: Offset) -> Option<Self> {
        if size.x > 0 && size.y > 0 {
            let width = size.x as usize;
            let height = size.y as usize;
            let mut buff = GdcBuffer::<u32>::alloc(height * width)?;
            let stride = width * 4;
            let bitmap = GdcBitmap::new_rgba8888(&mut buff, stride, size);
            Some(Self {
                buff,
                bitmap,
                viewport: Viewport::from_size(size),
            })
        } else {
            panic!();
        }
    }

    pub fn row(&mut self, row: i16) -> Option<&mut [u32]> {
        if row >= 0 && row < self.bitmap.height() {
            let offset = self.bitmap.stride() / 4 * row as usize;
            Some(&mut self.buff.data[offset..offset + self.bitmap.width() as usize])
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
    fn bitmap(&self) -> rgb::BitmapRef {
        rgb::BitmapRef::new(&self.bitmap)
    }

    fn blend_pixel(&mut self, _pt: Point, _color: Color, _alpha: u8) {
        // TODO: not implemented yet, requires 32-bit color blending routines
    }

    fn blur_rect(&mut self, _r: Rect, _radius: usize) {
        // TODO
    }
}
