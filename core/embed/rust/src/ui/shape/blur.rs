use crate::ui::{canvas::rgb::RgbCanvasEx, geometry::Rect};

use super::{DrawingContext, Renderer, Shape, ShapeClone};

use without_alloc::alloc::LocalAllocLeakExt;

// ==========================================================================
// struct Blur
// ==========================================================================

pub struct Blurring {
    area: Rect,
    radius: usize,
}

impl Blurring {
    pub fn new(area: Rect, radius: usize) -> Self {
        Self { area, radius }
    }

    pub fn render<'a>(self, renderer: &mut impl Renderer<'a>) {
        renderer.render_shape(self);
    }
}

impl Shape for Blurring {
    fn bounds(&self, _context: &mut DrawingContext) -> Rect {
        self.area
    }

    fn cleanup(&self, _context: &mut DrawingContext) {}

    fn draw(&self, canvas: &mut dyn RgbCanvasEx, _context: &mut DrawingContext) {
        canvas.blur_rect(self.area, self.radius);
    }
}

impl ShapeClone for Blurring {
    fn clone_at_pool<'alloc, T>(self, pool: &'alloc T) -> Option<&'alloc mut dyn Shape>
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        let clone = pool.alloc_t::<Blurring>()?;
        Some(clone.uninit.init(Blurring { ..self }))
    }
}
