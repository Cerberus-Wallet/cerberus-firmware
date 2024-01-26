use crate::ui::{
    canvas::rgb::{RgbCanvasEx, TextAttr},
    display::{Color, Font},
    geometry::Rect,
};

use super::{DrawingContext, Renderer, Shape, ShapeClone};

use without_alloc::alloc::LocalAllocLeakExt;

// ==========================================================================
// struct Text
// ==========================================================================

pub struct Text<'a> {
    area: Rect,
    text: &'a str,
    color: Color,
    font: Font,
}

impl<'a> Text<'a> {
    pub fn new(area: Rect, text: &'a str) -> Self {
        Self {
            area,
            text,
            color: Color::white(),
            font: Font::NORMAL,
        }
    }

    pub fn with_fg(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn with_font(self, font: Font) -> Self {
        Self { font, ..self }
    }

    pub fn render<'b>(self, renderer: &mut impl Renderer) {
        renderer.render_shape(self);
    }
}

impl<'a> Shape for Text<'a> {
    fn bounds(&self, _context: &mut dyn DrawingContext) -> Rect {
        // TODO:: returned value could be possibly smaller
        // (according to the text, font, offset and alignment)
        self.area
    }

    fn cleanup(&self, _context: &mut dyn DrawingContext) {}

    fn draw(&self, canvas: &mut dyn RgbCanvasEx, _context: &mut dyn DrawingContext) {
        let attr = TextAttr::new().with_fg(self.color).with_font(self.font);
        canvas.draw_text(self.area, &self.text, &attr);
    }
}

impl<'a> ShapeClone for Text<'a> {
    fn clone_at_pool<'alloc, T>(self, pool: &'alloc T) -> Option<&'alloc mut dyn Shape>
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        let clone = pool.alloc_t::<Text>()?;
        let text = pool.copy_str(self.text)?;
        Some(clone.uninit.init(Text { text, ..self }))
    }
}
