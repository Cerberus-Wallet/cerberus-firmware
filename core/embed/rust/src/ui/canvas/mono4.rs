use crate::ui::{canvas::mono, geometry::Offset};

use crate::trezorhal::gdc::{GdcBitmap, GdcBuffer};

// ==========================================================================
// 4-bpp Monochromatic Canvas
// ==========================================================================

pub struct MonoCanvas<'a> {
    buff: GdcBuffer<'a, u8>,
    bitmap: GdcBitmap<'a>,
}

impl<'a> MonoCanvas<'a> {
    pub fn new(size: Offset) -> Option<Self> {
        if size.x > 0 && size.y > 0 {
            let width = size.x as usize;
            let height = size.y as usize;
            let stride = (width + 1) / 2;
            let mut buff = GdcBuffer::<u8>::alloc(height * width)?;
            let bitmap = GdcBitmap::new_mono4(&mut buff, stride, size);
            Some(Self { buff, bitmap })
        } else {
            panic!();
        }
    }

    pub fn row(&mut self, row: i16) -> Option<&mut [u8]> {
        if row >= 0 && row < self.bitmap.height() {
            let offset = self.bitmap.stride() * row as usize;
            Some(&mut self.buff.data[offset..offset + ((self.bitmap.width() + 1) / 2) as usize])
        } else {
            None
        }
    }

    pub fn row_bytes(&mut self, row: i16, height: i16) -> Option<&mut [u8]> {
        if row >= 0
            && height > 0
            && row < self.bitmap.height()
            && row + height <= self.bitmap.height()
        {
            let offset = self.bitmap.stride() * row as usize;
            let len = self.bitmap.stride() * height as usize;
            Some(&mut self.buff.data[offset..offset + len])
        } else {
            None
        }
    }
}

impl<'a> mono::MonoCanvas for MonoCanvas<'a> {
    fn bitmap<'b>(&'b self) -> mono::BitmapRef<'b> {
        mono::BitmapRef::new(&self.bitmap)
    }

    fn size(&self) -> Offset {
        self.bitmap.size()
    }
}
