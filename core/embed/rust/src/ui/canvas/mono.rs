use crate::ui::geometry::{Offset, Rect};

use crate::trezorhal::gdc;

pub type BitmapRef<'a> = gdc::GdcBitmapRef<'a>;

/*pub struct MonoTextAttr {
    // luminance
    lum: u8,
    offset: Offset,
    // TODO: horizontal alignment
    // TODO: vertical alignment
}*/

/*pub enum MonoBitmapRef<'a> {
    // (&bitmap, offset, luminance)
    Mono1(&'a mono1::Canvas, Offset, u8),
    Mono4(&'a mono4::Canvas, Offset, u8),
}*/

pub trait MonoCanvas {
    /// Returns dimensions of the canvas in pixels
    fn size(&self) -> Offset;

    /// Returns a non-mutable reference to the underlying bitmap
    fn bitmap<'a>(&'a self) -> BitmapRef<'a>;

    /// Returns the dimensions of the canvas as a rectangle with the top-left at
    /// (0,0)
    fn bounds(&self) -> Rect {
        Rect::from_size(self.size())
    }

    fn width(&self) -> i16 {
        self.size().x
    }

    fn height(&self) -> i16 {
        self.size().y
    }
}
