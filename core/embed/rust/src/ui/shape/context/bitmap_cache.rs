use without_alloc::alloc::LocalAllocLeakExt;

pub struct BitmapCache<'a> {
    data: &'a [u8],
}

impl<'a> BitmapCache<'a> {
    pub fn new<'alloc: 'a, T>(_pool: &'alloc T) -> Self
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        BitmapCache { data: &[] }
    }
}
