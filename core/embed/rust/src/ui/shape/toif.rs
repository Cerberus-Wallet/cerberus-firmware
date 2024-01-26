use crate::ui::{
    canvas::{
        mono::MonoCanvas,
        mono4,
        rgb::{RgbCanvas, RgbCanvasEx},
        rgb565,
    },
    display::{toif::Toif, Color},
    geometry::{Offset, Point, Rect},
};

use super::{DrawingContext, Renderer, Shape, ShapeClone};

use without_alloc::alloc::LocalAllocLeakExt;

// ==========================================================================
// struct ToifImage
// ==========================================================================

// A rectangle filled with a solid color
pub struct ToifImage {
    pos: Point,
    toif: Toif<'static>,
    fg_color: Color,
    bg_color: Option<Color>,
}

impl ToifImage {
    pub fn new(pos: Point, toif: Toif<'static>) -> Self {
        Self {
            pos,
            toif,
            fg_color: Color::white(),
            bg_color: None,
        }
    }

    pub fn with_fg(self, fg_color: Color) -> Self {
        Self { fg_color, ..self }
    }

    pub fn with_bg(self, bg_color: Color) -> Self {
        Self {
            bg_color: Some(bg_color),
            ..self
        }
    }

    pub fn render<'b>(self, renderer: &mut impl Renderer<'b>) {
        renderer.render_shape(self);
    }

    fn draw_grayscale(&self, canvas: &mut dyn RgbCanvasEx, context: &mut dyn DrawingContext) {
        // TODO: introduce new viewport/shape function for this calculation
        let viewport = canvas.viewport();
        let mut clip = self
            .bounds(context)
            .intersect(viewport.clip.translate(-viewport.origin))
            .translate((-self.pos).into());

        // TODO: calculate optimal height of the slice
        let mut slice = mono4::MonoCanvas::new(Offset::new(self.toif.width(), 32)).unwrap();

        while !clip.is_empty() {
            let height = core::cmp::min(slice.height(), clip.height());
            context.deflate_toif(self.toif, clip.y0, slice.row_bytes(0, height).unwrap());

            let r = clip.translate(self.pos.into());

            // TODO: a bit strange here..
            let slice_ref = slice
                .bitmap()
                .with_fg(self.fg_color)
                .with_offset(Offset::new(r.x0 - self.pos.x, 0));

            match self.bg_color {
                Some(bg_color) => canvas.draw_bitmap(r, &slice_ref.with_bg(bg_color)),
                None => canvas.blend_bitmap(r, &slice_ref),
            }

            clip.y0 += height;
        }
    }

    fn draw_rgb(&self, canvas: &mut dyn RgbCanvasEx, context: &mut dyn DrawingContext) {
        // TODO: introduce new viewport/shape function for this calculation
        let viewport = canvas.viewport();
        let mut clip = self
            .bounds(context)
            .intersect(viewport.clip.translate(-viewport.origin))
            .translate((-self.pos).into());

        // TODO: calculate optimal height of the slice
        let mut slice = rgb565::Canvas::new(Offset::new(self.toif.width(), 8)).unwrap();

        while !clip.is_empty() {
            let height = core::cmp::min(slice.height(), clip.height());
            context.deflate_toif(self.toif, clip.y0, slice.row_bytes(0, height).unwrap());

            let r = clip.translate(self.pos.into());

            // TODO: a bit strange here..
            let slice_ref = slice
                .bitmap()
                .with_offset(Offset::new(r.x0 - self.pos.x, 0));

            canvas.draw_bitmap(r, &slice_ref);

            clip.y0 += height;
        }
    }
}

impl Shape for ToifImage {
    fn bounds(&self, _context: &mut dyn DrawingContext) -> Rect {
        let toif_size = Offset::new(self.toif.width(), self.toif.height());
        Rect::from_top_left_and_size(self.pos, toif_size)
    }

    fn cleanup(&self, _context: &mut dyn DrawingContext) {
        // TODO: inform the context that we won't use the zlib slot anymore
    }

    fn draw(&self, canvas: &mut dyn RgbCanvasEx, context: &mut dyn DrawingContext) {
        if self.toif.is_grayscale() {
            self.draw_grayscale(canvas, context);
        } else {
            self.draw_rgb(canvas, context);
        }
    }
}

impl ShapeClone for ToifImage {
    fn clone_at_pool<'alloc, T>(self, pool: &'alloc T) -> Option<&'alloc mut dyn Shape>
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        let clone = pool.alloc_t::<ToifImage>()?;
        Some(clone.uninit.init(ToifImage { ..self }))
    }
}
