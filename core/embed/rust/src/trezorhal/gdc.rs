use super::ffi;
use crate::ui::{
    display::{Color, Font},
    geometry::{Offset, Rect},
};
use core::{mem, slice};

#[derive(PartialEq, Debug, Eq, FromPrimitive, Clone, Copy)]
pub enum GdcFormat {
    MONO1 = ffi::gdc_format_t_GDC_FORMAT_MONO1 as _,
    MONO4 = ffi::gdc_format_t_GDC_FORMAT_MONO4 as _,
    RGB565 = ffi::gdc_format_t_GDC_FORMAT_RGB565 as _,
    RGBA8888 = ffi::gdc_format_t_GDC_FORMAT_RGBA8888 as _,
}

impl Into<Rect> for ffi::gdc_rect_t {
    fn into(self) -> Rect {
        Rect {
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,
        }
    }
}

impl Into<ffi::gdc_rect_t> for Rect {
    fn into(self) -> ffi::gdc_rect_t {
        ffi::gdc_rect_t {
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,
        }
    }
}

impl Into<Offset> for ffi::gdc_size_t {
    fn into(self) -> Offset {
        Offset {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<ffi::gdc_size_t> for Offset {
    fn into(self) -> ffi::gdc_size_t {
        ffi::gdc_size_t {
            x: self.x,
            y: self.y,
        }
    }
}

pub struct GdcBuffer<'a, T> {
    pub data: &'a mut [T],
    allocated: bool,
}

impl<'a, T> GdcBuffer<'a, T> {
    pub fn alloc(len: usize) -> Option<Self> {
        unsafe {
            let ptr = ffi::gdc_heap_alloc(len * mem::size_of::<T>());
            if !ptr.is_null() {
                let data = slice::from_raw_parts_mut(ptr as *mut T, len);
                Some(Self {
                    data,
                    allocated: true,
                })
            } else {
                None
            }
        }
    }
}

impl<'a, T> Drop for GdcBuffer<'a, T> {
    fn drop(&mut self) {
        if self.allocated {
            unsafe {
                ffi::gdc_heap_free(
                    self.data.as_mut_ptr() as *mut cty::c_void,
                    self.data.len() * mem::size_of::<T>(),
                );
            }
        }
    }
}

pub type GdcBitmap<'a> = ffi::gdc_bitmap_t;

impl<'a> GdcBitmap<'a> {
    pub fn new_rgb565<'b>(
        buff: &'b mut GdcBuffer<u16>,
        stride: usize,
        size: Offset,
    ) -> GdcBitmap<'b> {
        unsafe {
            ffi::gdc_bitmap_rgb565(
                buff.data.as_mut_ptr() as *mut cty::c_void,
                stride,
                size.into(),
                0,
            )
        }
    }

    pub fn from_rgb565_slice<'b>(buff: &'b [u16], stride: usize, size: Offset) -> GdcBitmap<'b> {
        unsafe {
            ffi::gdc_bitmap_rgb565(
                buff.as_ptr() as *mut cty::c_void,
                stride,
                size.into(),
                (ffi::GDC_BITMAP_READ_ONLY | ffi::GDC_BITMAP_NO_DMA) as u8,
            )
        }
    }

    pub fn new_rgba8888<'b>(
        buff: &'b mut GdcBuffer<u32>,
        stride: usize,
        size: Offset,
    ) -> GdcBitmap<'b> {
        unsafe {
            ffi::gdc_bitmap_rgba8888(
                buff.data.as_mut_ptr() as *mut cty::c_void,
                stride,
                size.into(),
                0,
            )
        }
    }

    pub fn new_mono1<'b>(
        buff: &'b mut GdcBuffer<u8>,
        stride: usize,
        size: Offset,
    ) -> GdcBitmap<'b> {
        ffi::gdc_bitmap_t {
            ptr: buff.data.as_mut_ptr() as *mut cty::c_void,
            stride: stride,
            size: size.into(),
            format: ffi::gdc_format_t_GDC_FORMAT_MONO1,
            vmt: core::ptr::null_mut(),
            attrs: 0,
        }
    }

    pub fn new_mono4<'b>(
        buff: &'b mut GdcBuffer<u8>,
        stride: usize,
        size: Offset,
    ) -> GdcBitmap<'b> {
        ffi::gdc_bitmap_t {
            ptr: buff.data.as_mut_ptr() as *mut cty::c_void,
            stride: stride,
            size: size.into(),
            format: ffi::gdc_format_t_GDC_FORMAT_MONO4,
            vmt: core::ptr::null_mut(),
            attrs: 0,
        }
    }

    pub fn width(&self) -> i16 {
        self.size.x
    }

    pub fn height(&self) -> i16 {
        self.size.y
    }

    pub fn size(&self) -> Offset {
        self.size.into()
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn gdc(&'a mut self) -> Gdc<'a> {
        unsafe {
            // let handle = core::mem::transmute::<&*mut ffi::gdc_vmt, &mut
            // cty::c_void>(&self.vmt);
            let handle = core::mem::transmute::<&mut ffi::gdc_bitmap_t, &mut cty::c_void>(self);
            Gdc { handle }
        }
    }
}

pub type GdcBitmapRef<'a> = ffi::gdc_bitmap_ref_t;

impl<'a> GdcBitmapRef<'a> {
    pub fn new<'b>(bitmap: &'b GdcBitmap) -> GdcBitmapRef<'b> {
        GdcBitmapRef {
            bitmap: &*bitmap,
            offset: Offset::zero().into(),
            fg_color: 0,
            bg_color: 0,
        }
    }

    pub fn with_offset(self, offset: Offset) -> Self {
        Self {
            offset: (offset + self.offset.into()).into(),
            ..self
        }
    }

    pub fn with_fg(self, fg_color: Color) -> Self {
        Self {
            fg_color: fg_color.into(),
            ..self
        }
    }

    pub fn with_bg(self, bg_color: Color) -> Self {
        Self {
            bg_color: bg_color.into(),
            ..self
        }
    }

    pub fn size(&self) -> Offset {
        unsafe { (&*self.bitmap).size() }
    }

    pub fn width(&self) -> i16 {
        unsafe { (&*self.bitmap).width() }
    }

    pub fn height(&self) -> i16 {
        unsafe { (&*self.bitmap).height() }
    }
}

pub type GdcTextAttr = ffi::gdc_text_attr_t;

impl GdcTextAttr {
    pub fn new() -> Self {
        Self {
            font: Font::NORMAL.into(),
            fg_color: Color::white().into(),
            bg_color: Color::black().into(),
            offset: Offset::zero().into(),
        }
    }

    pub fn with_font(self, font: Font) -> Self {
        Self {
            font: font.into(),
            ..self
        }
    }

    pub fn with_fg(self, fg_color: Color) -> Self {
        Self {
            fg_color: fg_color.into(),
            ..self
        }
    }

    pub fn with_bg(self, bg_color: Color) -> Self {
        Self {
            bg_color: bg_color.into(),
            ..self
        }
    }

    pub fn with_offset(self, offset: Offset) -> Self {
        Self {
            offset: (offset + self.offset.into()).into(),
            ..self
        }
    }
}

pub struct Gdc<'a> {
    handle: &'a mut ffi::gdc_t,
}

impl<'a> Gdc<'a> {
    pub fn size(&self) -> Offset {
        unsafe { ffi::gdc_get_size(self.handle).into() }
    }

    pub fn set_window_hint(&mut self, rect: Rect) {
        unsafe { ffi::gdc_set_window_hint(self.handle, rect.into()) }
    }

    // Draw filled rectangle
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        unsafe {
            let ok = ffi::gdc_fill_rect(self.handle, rect.into(), color.into());
            assert!(ok);
        }
    }

    pub fn draw_bitmap(&mut self, rect: Rect, src: &GdcBitmapRef) {
        unsafe {
            let ok = ffi::gdc_draw_bitmap(self.handle, rect.into(), src);
            assert!(ok);
        }
    }

    pub fn draw_blended(&mut self, rect: Rect, bg: &GdcBitmapRef, fg: &GdcBitmapRef) {
        unsafe {
            let ok = ffi::gdc_draw_blended(self.handle, rect.into(), bg, fg);
            assert!(ok);
        }
    }

    pub fn draw_opaque_text(&mut self, rect: Rect, text: &str, attr: &GdcTextAttr) {
        unsafe {
            let ok = ffi::gdc_draw_opaque_text(
                self.handle,
                rect.into(),
                text.as_ptr() as _,
                text.len(),
                attr,
            );
            assert!(ok);
        }
    }

    pub fn draw_blended_text(
        &mut self,
        rect: Rect,
        text: &str,
        attr: &GdcTextAttr,
        bg: &GdcBitmapRef,
    ) {
        unsafe {
            let ok = ffi::gdc_draw_blended_text(
                self.handle,
                rect.into(),
                text.as_ptr() as _,
                text.len(),
                attr,
                bg,
            );
            assert!(ok);
        }
    }
}

impl<'a> Drop for Gdc<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::gdc_release(self.handle);
        }
    }
}

pub struct Display {}

impl Display {
    pub fn acquire_gdc<'a>() -> Option<Gdc<'a>> {
        unsafe {
            let handle = ffi::display_acquire_gdc();
            if !handle.is_null() {
                Some(Gdc {
                    handle: &mut *handle,
                })
            } else {
                None
            }
        }
    }
}
