use crate::ui::{
    canvas::Canvas,
    display::Color,
    geometry::{Offset, Point, Rect},
};

use super::{DrawingCache, Renderer, Shape, ShapeClone};

use without_alloc::alloc::LocalAllocLeakExt;

/// A shape for the a 'snake'
pub struct Snake {
    /// Position of point (0,0)
    pos: Point,
    /// Snake points
    snake: &'static [(i16, i16)],
    /// Color
    color: Color,
    /// Scale (length of square size)
    scale: i16,
}

impl Snake {
    pub fn new(pos: Point, snake: &'static [(i16, i16)]) -> Self {
        Self {
            pos,
            snake,
            color: Color::white(),
            scale: 2,
        }
    }

    pub fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn with_scale(self, scale: i16) -> Self {
        Self { scale, ..self }
    }

    fn cell_rect(&self, pt: (i16, i16)) -> Rect {
        let pt = Point::new(
            self.pos.x + (pt.0 * self.scale) - self.scale / 2,
            self.pos.y + (pt.1 * self.scale) - self.scale / 2,
        );
        Rect::from_top_left_and_size(pt, Offset::uniform(self.scale))
    }

    pub fn render(self, renderer: &mut impl Renderer) {
        renderer.render_shape(self);
    }
}

impl Shape for Snake {
    fn bounds(&self, _cache: &DrawingCache) -> Rect {
        if self.snake.is_empty() {
            Rect::zero()
        } else {
            let mut b = self.cell_rect(self.snake[0]);
            self.snake[1..]
                .iter()
                .for_each(|c| b = b.intersect(self.cell_rect(*c)));
            b
        }
    }

    fn cleanup(&mut self, _cache: &DrawingCache) {}

    fn draw(&mut self, canvas: &mut dyn Canvas, _cache: &DrawingCache) {
        for c in self.snake.iter() {
            canvas.fill_rect(self.cell_rect(*c), self.color);
        }
    }
}

impl ShapeClone for Snake {
    fn clone_at_bump<'alloc, T>(self, bump: &'alloc T) -> Option<&'alloc mut dyn Shape>
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        let clone = bump.alloc_t::<Snake>()?;
        Some(clone.uninit.init(Snake { ..self }))
    }
}
