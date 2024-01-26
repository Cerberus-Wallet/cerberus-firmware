/// This module implements the `Circle` shape for rendering
/// filled circles, circles with outlines and specified thickness,
/// or just the outline of a circle.
use crate::ui::{
    canvas::rgb::RgbCanvasEx,
    display::Color,
    geometry::{Point, Rect},
};

use super::{DrawingContext, Renderer, Shape, ShapeClone};

use without_alloc::alloc::LocalAllocLeakExt;

pub struct Circle {
    center: Point,
    radius: i16,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    thickness: i16,
}

impl Circle {
    pub fn new(center: Point, radius: i16) -> Self {
        Self {
            center,
            radius,
            fg_color: None,
            bg_color: None,
            thickness: 1,
        }
    }

    pub fn with_fg(self, fg_color: Color) -> Self {
        Self {
            fg_color: Some(fg_color),
            ..self
        }
    }

    pub fn with_bg(self, bg_color: Color) -> Self {
        Self {
            bg_color: Some(bg_color),
            ..self
        }
    }

    pub fn with_thickness(self, thickness: i16) -> Self {
        Self {
            thickness: thickness,
            ..self
        }
    }

    pub fn render<'a>(self, renderer: &mut impl Renderer<'a>) {
        renderer.render_shape(self);
    }
}

impl Shape for Circle {
    fn bounds(&self, _context: &mut dyn DrawingContext) -> Rect {
        let c = self.center;
        let r = self.radius;
        Rect::new(
            Point::new(c.x - r, c.y - r),
            Point::new(c.x + r + 1, c.y + r + 1),
        )
    }

    fn cleanup(&self, _context: &mut dyn DrawingContext) {}

    fn draw(&self, canvas: &mut dyn RgbCanvasEx, _context: &mut dyn DrawingContext) {
        // NOTE: drawing of circles without a background and with a thickness > 1
        //       is not supported. If we needed it, we would have to
        //       introduce RgbCanvas::draw_ring() function.

        // TODO: panic! in unsupported scenarious

        let th = match self.fg_color {
            Some(_) => self.thickness,
            None => 0,
        };

        if self.thickness == 1 {
            if let Some(color) = self.bg_color {
                canvas.fill_circle_aa(self.center, self.radius, color);
            }
            if let Some(color) = self.fg_color {
                canvas.draw_circle_aa(self.center, self.radius, color);
            }
        } else {
            if let Some(color) = self.fg_color {
                canvas.fill_circle_aa(self.center, self.radius, color);
            }
            if let Some(color) = self.bg_color {
                canvas.fill_circle_aa(self.center, self.radius - th, color);
            }
        }
    }
}

impl ShapeClone for Circle {
    fn clone_at_pool<'alloc, T>(self, pool: &'alloc T) -> Option<&'alloc mut dyn Shape>
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        let clone = pool.alloc_t::<Circle>()?;
        Some(clone.uninit.init(Circle { ..self }))
    }
}
