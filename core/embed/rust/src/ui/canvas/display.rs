use crate::ui::{
    canvas::{rgb, Viewport},
    display::Color,
    geometry::{Offset, Rect},
};

use crate::trezorhal::gdc::{Display, Gdc};

// ==========================================================================
// Display canvas
// ==========================================================================

pub struct Canvas<'a> {
    gdc: Gdc<'a>,
    viewport: Viewport,
}

impl<'a> Canvas<'a> {
    pub fn acquire() -> Option<Self> {
        let gdc = Display::acquire_gdc()?;
        let viewport = Viewport::from_size(gdc.size());
        Some(Self { gdc, viewport })
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
        self.gdc.size()
    }

    fn fill_rect(&mut self, r: Rect, color: Color) {
        let new_r = r
            .translate(self.viewport.origin)
            .intersect(self.viewport.clip);
        self.gdc.fill_rect(new_r, color);
    }

    fn draw_bitmap(&mut self, r: Rect, bitmap: &rgb::BitmapRef) {
        let r_moved = r.translate(self.viewport.origin);
        let r_clipped = r_moved.intersect(self.viewport.clip);
        let new_b = bitmap
            .clone()
            .with_offset(r_clipped.top_left() - r_moved.top_left());
        self.gdc.draw_bitmap(r_clipped, &new_b);
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
        self.gdc.draw_blended(r_clipped, &new_fg, &new_bg);
    }

    fn draw_opaque_text(&mut self, r: Rect, text: &str, attr: &rgb::TextAttr) {
        let r_moved = r.translate(self.viewport.origin);
        let r_clipped = r_moved.intersect(self.viewport.clip);
        let new_attr = attr
            .clone()
            .with_offset(r_clipped.top_left() - r_moved.top_left());
        self.gdc.draw_opaque_text(r_clipped, text, &new_attr);
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
        self.gdc
            .draw_blended_text(r_clipped, text, &new_attr, &new_bg);
    }
}
