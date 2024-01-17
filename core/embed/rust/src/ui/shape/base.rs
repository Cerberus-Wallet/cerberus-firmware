use crate::ui::{canvas::rgb::RgbCanvasEx, geometry::Rect};

use super::DrawingContext;

use without_alloc::alloc::LocalAllocLeakExt;

// ==========================================================================
// trait Shape
// ==========================================================================

/// All shapes (like `Bar`, `Text`, `Icon`, ...) that can be rendered
/// must implement Shape. This trait is used internally
/// by so-called Renderers - `DirectRenderer` & `ProgressiveRederer`. Shape
/// objects may use DrawingContext as a scratch-pad memory or for caching
/// expensive calculations results.
pub trait Shape {
    /// Returns the smallest bounding rectangle containing whole parts of the
    /// shape This method is used by renderer for optimization if the shape
    /// must be renderer or not
    fn bounds(&self, context: &mut DrawingContext) -> Rect;
    /// Draws shape on the canvas
    fn draw(&self, canvas: &mut dyn RgbCanvasEx, context: &mut DrawingContext);
    /// Releases data allocated in context memory
    /// Is called by renderer if the shape draw() function won't be called
    /// anymore
    fn cleanup(&self, context: &mut DrawingContext);
}

// ==========================================================================
// trait ShapeClone
// ==========================================================================

/// All shapes (like `Bar`, `Text`, `Icon`, ...) that can be rendered
/// by `ProgressiveRender` must implement `ShapeClone`.
pub trait ShapeClone {
    /// Clone a shape object at the specified memory pool
    /// This method is used by renderers to store shape objects for deferred
    /// drawing
    fn clone_at_pool<'alloc, T>(self, pool: &'alloc T) -> Option<&'alloc mut dyn Shape>
    where
        T: LocalAllocLeakExt<'alloc>;
}
