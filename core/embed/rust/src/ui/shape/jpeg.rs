use crate::ui::{
    canvas::rgb::RgbCanvasEx,
    geometry::{Point, Rect},
};

use super::{DrawingContext, JpegFnOutput, Renderer, Shape, ShapeClone};

use without_alloc::alloc::LocalAllocLeakExt;

pub struct JpegImage {
    pos: Point,
    blur_radius: usize,
    jpeg: &'static [u8],
}

impl JpegImage {
    pub fn new(pos: Point, jpeg: &'static [u8]) -> Self {
        JpegImage {
            pos,
            blur_radius: 0,
            jpeg,
        }
    }

    pub fn with_blur(self, blur_radius: usize) -> Self {
        Self {
            blur_radius: blur_radius,
            ..self
        }
    }

    pub fn render(self, renderer: &mut impl Renderer) {
        renderer.render_shape(self);
    }
}

impl Shape for JpegImage {
    fn bounds(&self, context: &mut dyn DrawingContext) -> Rect {
        // TODO: consider caching size inside JpegImage structure
        //      (this unfortunately requires &mut self here, or adding another trait
        // method init())

        // TODO:: replace unwrap!
        let size = unwrap!(context.get_jpeg_size(self.jpeg));
        Rect::from_top_left_and_size(self.pos, size)
    }

    fn cleanup(&self, _context: &mut dyn DrawingContext) {}

    fn draw(&self, canvas: &mut dyn RgbCanvasEx, context: &mut dyn DrawingContext) {
        let clip = canvas.viewport().relative_clip(self.bounds(context)).clip;

        // translate clip to JPEG relative coordinates
        let clip = clip.translate(-canvas.viewport().origin);
        let clip = clip.translate((-self.pos).into());

        unwrap!(context.decompress_jpeg(
            self.jpeg,
            clip.top_left(),
            &mut JpegFnOutput::new(|mcu_r, mcu_bitmap| {
                // draw mcu (might be clipped if needed)
                canvas.draw_bitmap(mcu_r.translate(self.pos.into()), mcu_bitmap);
                // Return true if we are not done yet
                mcu_r.x1 < clip.x1 || mcu_r.y1 < clip.y1
            })
        ));

        // TODO: add blurring variant
    }
}

impl ShapeClone for JpegImage {
    fn clone_at_pool<'alloc, T>(self, pool: &'alloc T) -> Option<&'alloc mut dyn Shape>
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        let clone = pool.alloc_t::<JpegImage>()?;
        Some(clone.uninit.init(JpegImage { ..self }))
    }
}
