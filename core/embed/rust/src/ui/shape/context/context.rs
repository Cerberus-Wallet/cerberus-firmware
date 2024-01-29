use super::{
    bitmap_cache::BitmapCache, blur_cache::BlurCache, jpeg_cache::JpegCache, zlib_cache::ZlibCache,
};

use crate::ui::{
    display::{tjpgd, toif::Toif},
    geometry::{Offset, Point},
};

use trezor_tjpgdec::JpegOutput;

use without_alloc::alloc::LocalAllocLeakExt;

pub trait DrawingContext<'a> {
    fn deflate_toif(&mut self, toif: Toif<'static>, from_row: i16, dest_buf: &mut [u8]);

    fn get_jpeg_size<'i: 'a>(&mut self, jpeg: &'i [u8]) -> Result<Offset, tjpgd::Error>;

    fn decompress_jpeg<'i: 'a>(
        &mut self,
        jpeg: &'i [u8],
        offset: Point,
        output: &mut dyn JpegOutput,
    ) -> Result<(), tjpgd::Error>;
}

pub struct DrawingContextImpl<'alloc> {
    zlib_cache: ZlibCache<'alloc>,
    jpeg_cache: JpegCache<'alloc>,
    blur_cache: BlurCache<'alloc>,
    bitmap_cache: BitmapCache<'alloc>,
}

impl<'alloc> DrawingContextImpl<'alloc> {
    pub fn new<TA>(pool_a: &'alloc TA, pool_b: &'alloc TA) -> Self
    where
        TA: LocalAllocLeakExt<'alloc>,
    {
        Self {
            zlib_cache: ZlibCache::new(pool_a, 4),
            jpeg_cache: JpegCache::new(pool_a, 1),
            blur_cache: BlurCache::new(pool_a),
            bitmap_cache: BitmapCache::new(pool_b),
        }
    }
}

impl<'a> DrawingContext<'a> for DrawingContextImpl<'a> {
    fn deflate_toif(&mut self, toif: Toif<'static>, from_row: i16, dest_buf: &mut [u8]) {
        let from_offset = toif.stride() * from_row as usize;
        self.zlib_cache
            .uncompress(toif.zdata(), from_offset, dest_buf)
            .unwrap();
    }

    fn get_jpeg_size<'i: 'a>(&mut self, jpeg: &'i [u8]) -> Result<Offset, tjpgd::Error> {
        self.jpeg_cache.get_size(jpeg)
    }

    fn decompress_jpeg<'i: 'a>(
        &mut self,
        jpeg: &'i [u8],
        offset: Point,
        output: &mut dyn JpegOutput,
    ) -> Result<(), tjpgd::Error> {
        self.jpeg_cache.decompress(jpeg, offset, output)
    }
}
