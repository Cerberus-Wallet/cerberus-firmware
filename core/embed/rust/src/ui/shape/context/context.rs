use super::{blur_cache::BlurCache, jpeg_cache::JpegCache, zlib_cache::ZlibCache};

use crate::ui::{
    canvas::rgb::BitmapRef,
    display::{tjpgd, toif::Toif},
    geometry::{Offset, Point, Rect},
};

use without_alloc::alloc::LocalAllocLeakExt;

pub struct DrawingContext<'a> {
    zlib_cache: ZlibCache<'a>,
    jpeg_cache: JpegCache<'a>,
    blur_cache: BlurCache<'a>,
    //bitmap_cache: BitmapCache<'a>
}

impl<'a> DrawingContext<'a> {
    pub fn new<T>(pool: &'a T) -> Self
    where
        T: LocalAllocLeakExt<'a>,
    {
        Self {
            zlib_cache: ZlibCache::new(pool, 4),
            jpeg_cache: JpegCache::new(pool, 1),
            blur_cache: BlurCache::new(pool),
            //bitmap_cache: BitmapCache::new(pool_dma),
        }
    }

    pub fn deflate_toif(&mut self, toif: Toif<'static>, from_row: i16, dest_buf: &mut [u8]) {
        let from_offset = toif.stride() * from_row as usize;
        self.zlib_cache
            .uncompress(toif.zdata(), from_offset, dest_buf)
            .unwrap();
    }

    pub fn get_jpeg_size<'i: 'a>(&mut self, jpeg: &'i [u8]) -> Result<Offset, tjpgd::Error> {
        self.jpeg_cache.get_size(jpeg)
    }

    pub fn decompress_jpeg<'i: 'a, F>(
        &mut self,
        jpeg: &'i [u8],
        offset: Point,
        output: F,
    ) -> Result<(), tjpgd::Error>
    where
        F: FnMut(Rect, &BitmapRef) -> bool,
    {
        self.jpeg_cache.decompress(jpeg, offset, output)
    }
}
