use crate::ui::{
    canvas::{
        rgb,
        rgb::{RgbCanvas, RgbCanvasEx},
        rgb565, Viewport,
    },
    display::{toif::Toif, Color},
    geometry::{Offset, Point, Rect},
    shape::{DrawingContext, Shape, ShapeClone},
};

use without_alloc::{alloc::LocalAllocLeakExt, FixedVec};

// ==========================================================================
// trait Renderer
// ==========================================================================

/// All renders must implement Renderer trait
/// Renderers can immediately use the draw() method of the passed shape or
/// may store it (using the boxed() method) and draw it later
pub trait Renderer {
    fn viewport(&self) -> Viewport;

    fn set_viewport(&mut self, viewport: Viewport);

    fn set_window(&mut self, window: Rect) -> Viewport {
        let viewport = self.viewport();
        self.set_viewport(viewport.relative_window(window));
        viewport
    }

    fn set_clip(&mut self, clip: Rect) -> Viewport {
        let viewport = self.viewport();
        self.set_viewport(viewport.relative_clip(clip));
        viewport
    }

    fn render_shape<S>(&mut self, shape: S)
    where
        S: Shape + ShapeClone;
}

// ==========================================================================
// struct DirectRenderer
// ==========================================================================

/// A simple implementation of a Renderer that draws directly onto the CanvasEx
pub struct DirectRenderer<'can, 'ctx, 'alloc: 'ctx> {
    /// Target canvas
    canvas: &'can mut dyn rgb::RgbCanvasEx,
    /// Drawing context (decompression context, scratch-pad memory)
    drawing_context: &'ctx mut dyn DrawingContext<'alloc>,
}

impl<'can, 'ctx, 'alloc> DirectRenderer<'can, 'ctx, 'alloc> {
    /// Creates a new DirectRenderer instance with the given canvas
    pub fn new<T>(
        canvas: &'can mut dyn rgb::RgbCanvasEx,
        bg_color: Option<Color>,
        context: &'ctx mut dyn DrawingContext<'alloc>,
    ) -> Self
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        if let Some(color) = bg_color {
            canvas.fill_background(color);
        }

        // TODO: consider storing original canvas.viewport
        //       and restoring it by drop() function

        Self {
            canvas,
            drawing_context: context,
        }
    }
}

impl<'can, 'ctx, 'alloc> Renderer for DirectRenderer<'can, 'ctx, 'alloc> {
    fn viewport(&self) -> Viewport {
        self.canvas.viewport()
    }

    fn set_viewport(&mut self, viewport: Viewport) {
        self.canvas.set_viewport(viewport);
    }

    fn render_shape<S>(&mut self, shape: S)
    where
        S: Shape + ShapeClone,
    {
        if self
            .canvas
            .viewport()
            .contains(shape.bounds(self.drawing_context))
        {
            shape.draw(self.canvas, self.drawing_context);
            shape.cleanup(self.drawing_context);
        }
    }
}

// ==========================================================================
// struct ProgressiveRenderer
// ==========================================================================

struct ShapeHolder<'a> {
    shape: &'a dyn Shape,
    viewport: Viewport,
}

/// A more advanced Renderer implementation that supports deferred rendering.
pub struct ProgressiveRenderer<'a, 'alloc, T: LocalAllocLeakExt<'alloc>> {
    /// Target canvas
    canvas: &'a mut dyn rgb::RgbCanvas,
    /// Pool for cloning shapes
    pool: &'alloc T,
    /// List of rendered shapes
    shapes: FixedVec<'alloc, ShapeHolder<'alloc>>,
    /// Current viewport
    viewport: Viewport,
    // Default background color
    bg_color: Option<Color>,
    /// Drawing context (decompression context, scratch-pad memory)
    drawing_context: &'a mut dyn DrawingContext<'alloc>,
}

impl<'a, 'alloc, T> ProgressiveRenderer<'a, 'alloc, T>
where
    T: LocalAllocLeakExt<'alloc>,
{
    /// Creates a new ProgressiveRenderer instance
    pub fn new(
        canvas: &'a mut dyn rgb::RgbCanvas,
        bg_color: Option<Color>,
        context: &'a mut dyn DrawingContext<'alloc>,
        pool: &'alloc T,
        max_shapes: usize,
    ) -> Self {
        let viewport = canvas.viewport();
        Self {
            canvas,
            pool,
            shapes: pool.fixed_vec(max_shapes).unwrap(),
            viewport,
            bg_color,
            drawing_context: context,
        }
    }

    /// Renders the stored shapes onto the specified canvas
    pub fn render(&mut self, lines: usize) {
        let canvas_clip = self.canvas.viewport().clip;
        let canvas_origin = self.canvas.viewport().origin;

        let mut slice =
            rgb565::Canvas::new(Offset::new(canvas_clip.width(), lines as i16)).unwrap();

        for y in (canvas_clip.y0..canvas_clip.y1).step_by(lines) {
            // Calculate the coordinates of the slice we will draw into
            let slice_r = Rect::new(
                // slice_r is in absolute coordinates
                Point::new(canvas_clip.x0, y),
                Point::new(canvas_clip.x1, y + lines as i16),
            )
            .translate(-canvas_origin);

            // Clear the slice background
            if let Some(color) = self.bg_color {
                slice.set_viewport(Viewport::from_size(slice_r.size()));
                slice.fill_background(color);
            }

            // Draw all shapes that overlaps the slice
            for holder in self.shapes.iter() {
                let shape_viewport = holder.viewport.absolute_clip(slice_r);
                let shape = holder.shape;
                let shape_bounds = shape.bounds(self.drawing_context);

                // Is the shape overlapping the current slice?
                if shape_viewport.contains(shape_bounds) {
                    slice.set_viewport(shape_viewport.translate((-slice_r.top_left()).into()));
                    shape.draw(&mut slice, self.drawing_context);

                    if shape_bounds.y1 + shape_viewport.origin.y <= shape_viewport.clip.y1 {
                        // The shape will never be drawn again
                        shape.cleanup(self.drawing_context);
                    }
                }
            }
            self.canvas.draw_bitmap(slice_r, &slice.bitmap());
        }
    }
}

impl<'a, 'alloc, T> Renderer for ProgressiveRenderer<'a, 'alloc, T>
where
    T: LocalAllocLeakExt<'alloc>,
{
    fn viewport(&self) -> Viewport {
        self.viewport
    }

    fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport.absolute_clip(self.canvas.bounds());
    }

    fn render_shape<S>(&mut self, shape: S)
    where
        S: Shape + ShapeClone,
    {
        // Is the shape visible?
        if self.viewport.contains(shape.bounds(self.drawing_context)) {
            // Clone the shape & push it to the list
            let holder = ShapeHolder {
                shape: shape.clone_at_pool(self.pool).unwrap(),
                viewport: self.viewport,
            };
            unwrap!(self.shapes.push(holder));
        }
    }
}

// ---------------------------------
// following code should be moved

impl<'i> Toif<'i> {
    // TODO: move to display::toif !!!
    pub fn stride(&self) -> usize {
        if self.is_grayscale() {
            self.width() as usize / 2
        } else {
            self.width() as usize * 2
        }
    }
}

impl Rect {
    // TODO:  move to geometry.rs !!!
    pub fn from_size(size: Offset) -> Self {
        Rect::from_top_left_and_size(Point::zero(), size)
    }

    pub fn has_intersection(&self, r: Rect) -> bool {
        self.x0 < r.x1 && self.x1 > r.x0 && self.y0 < r.y1 && self.y1 > r.y0
    }

    pub fn intersect(&self, r: Rect) -> Rect {
        Rect::new(
            Point::new(core::cmp::max(self.x0, r.x0), core::cmp::max(self.y0, r.y0)),
            Point::new(core::cmp::min(self.x1, r.x1), core::cmp::min(self.y1, r.y1)),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.x0 >= self.x1 || self.y0 >= self.y1
    }
}

impl core::ops::Neg for Point {
    // TODO:  move to geometry.rs !!!
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}
