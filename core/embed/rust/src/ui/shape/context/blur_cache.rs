use without_alloc::alloc::LocalAllocLeakExt;

pub struct BlurCache<'a> {
    data: &'a [u8],
}

impl<'a> BlurCache<'a> {
    pub fn new<'alloc: 'a, T>(_pool: &'alloc T) -> Self
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        BlurCache { data: &[] }
    }
}
