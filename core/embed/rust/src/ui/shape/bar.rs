/// This module implements `Bar` shape for rendering various
/// types of rectangles. These rectangle might have optional
/// rounded corners and outline.
use crate::ui::{
    canvas::rgb::RgbCanvasEx,
    display::Color,
    geometry::{Insets, Rect},
};

use super::{DrawingContext, Renderer, Shape, ShapeClone};

use without_alloc::alloc::LocalAllocLeakExt;

pub struct Bar {
    area: Rect,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    thickness: i16,
    radius: i16,
}

impl Bar {
    pub fn new(area: Rect) -> Self {
        Self {
            area,
            fg_color: None,
            bg_color: None,
            thickness: 1,
            radius: 0,
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

    pub fn with_radius(self, radius: i16) -> Self {
        Self { radius, ..self }
    }

    pub fn with_thickness(self, thickness: i16) -> Self {
        Self { thickness, ..self }
    }

    pub fn render<'a>(self, renderer: &mut impl Renderer<'a>) {
        renderer.render_shape(self);
    }
}

impl Shape for Bar {
    fn bounds(&self, _context: &mut dyn DrawingContext) -> Rect {
        self.area
    }

    fn cleanup(&self, _context: &mut dyn DrawingContext) {}

    fn draw(&self, canvas: &mut dyn RgbCanvasEx, _context: &mut dyn DrawingContext) {
        // NOTE: drawing of rounded bars without a background
        //       is not supported. If we needed it, we would have to
        //       introduce a new function in RgbCanvas.

        // TODO: panic! in unsupported scenarious

        let th = match self.fg_color {
            Some(_) => self.thickness,
            None => 0,
        };

        if self.radius == 0 {
            if let Some(fg_color) = self.fg_color {
                // outline
                let r = self.area;
                canvas.fill_rect(Rect { y1: r.y0 + th, ..r }, fg_color);
                canvas.fill_rect(Rect { x1: r.x0 + th, ..r }, fg_color);
                canvas.fill_rect(Rect { x0: r.x1 - th, ..r }, fg_color);
                canvas.fill_rect(Rect { y0: r.y1 - th, ..r }, fg_color);
            }
            if let Some(bg_color) = self.bg_color {
                // background
                let bg_r = self.area.inset(Insets::uniform(th));
                canvas.fill_rect(bg_r, bg_color);
            }
        } else {
            if let Some(fg_color) = self.fg_color {
                canvas.fill_round_rect(self.area, self.radius, fg_color);
            }
            if let Some(bg_color) = self.bg_color {
                let bg_r = self.area.inset(Insets::uniform(th));
                canvas.fill_round_rect_aa(bg_r, self.radius, bg_color);
            }
        }
    }
}

impl ShapeClone for Bar {
    fn clone_at_pool<'alloc, T>(self, pool: &'alloc T) -> Option<&'alloc mut dyn Shape>
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        let clone = pool.alloc_t::<Bar>()?;
        Some(clone.uninit.init(Bar { ..self }))
    }
}
