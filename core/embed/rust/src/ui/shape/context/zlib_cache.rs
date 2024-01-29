use crate::trezorhal::uzlib::{UzlibContext, UZLIB_WINDOW_SIZE};
use core::cell::UnsafeCell;
use without_alloc::{alloc::LocalAllocLeakExt, FixedVec};

struct ZlibCacheSlot<'a> {
    // Reference to compressed data
    zdata: &'a [u8],
    // Current offset in docempressed data
    offset: usize,
    // Decompression context for the current zdata
    dc: Option<UzlibContext<'a>>,
    // Window used by current decompression context
    // (it's used just by own dc and nobody else)
    window: &'a UnsafeCell<[u8; UZLIB_WINDOW_SIZE]>,
}

impl<'a> ZlibCacheSlot<'a> {
    fn new<'alloc: 'a, T>(pool: &'alloc T) -> ZlibCacheSlot<'a>
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        let window = unwrap!(pool.alloc_t::<UnsafeCell<[u8; UZLIB_WINDOW_SIZE]>>())
            .uninit
            .init(UnsafeCell::new([0; UZLIB_WINDOW_SIZE]));

        ZlibCacheSlot {
            zdata: &[],
            offset: 0,
            dc: None,
            window,
        }
    }

    // May be called with zdata == &[] to make the slot free
    fn reset(&mut self, zdata: &'static [u8]) {
        // Drop the existing decompression context holding
        // a mutable reference to window buffer
        self.dc = None;

        if zdata.len() > 0 {
            // Now there's nobody else holding any reference to our window
            // so we can get mutable reference and pass it to a new
            // instance of the decompression context
            let window = unsafe { &mut *self.window.get() };

            self.dc = Some(UzlibContext::new(zdata, Some(window)));
        }

        self.offset = 0;
        self.zdata = zdata;
    }

    fn uncompress(&mut self, dest_buf: &mut [u8]) -> Result<bool, ()> {
        if let Some(dc) = self.dc.as_mut() {
            match dc.uncompress(dest_buf) {
                Ok(done) => {
                    if done {
                        self.reset(&[]);
                    } else {
                        self.offset += dest_buf.len();
                    }
                    Ok(done)
                }
                Err(e) => Err(e),
            }
        } else {
            Err(())
        }
    }

    fn skip(&mut self, nbytes: usize) -> Result<bool, ()> {
        if let Some(dc) = self.dc.as_mut() {
            match dc.skip(nbytes) {
                Ok(done) => {
                    if done {
                        self.reset(&[]);
                    } else {
                        self.offset += nbytes;
                    }

                    Ok(done)
                }
                Err(e) => Err(e),
            }
        } else {
            Err(())
        }
    }

    fn is_for<'b>(&self, zdata: &'b [u8], offset: usize) -> bool {
        self.zdata == zdata && self.offset == offset
    }
}

pub struct ZlibCache<'a> {
    slots: FixedVec<'a, ZlibCacheSlot<'a>>,
}

impl<'a> ZlibCache<'a> {
    pub fn new<'alloc: 'a, T>(pool: &'alloc T, slot_count: usize) -> Self
    where
        T: LocalAllocLeakExt<'alloc>,
    {
        let mut cache = ZlibCache {
            slots: unwrap!(pool.fixed_vec(slot_count)),
        };

        for _ in 0..cache.slots.capacity() {
            unwrap!(cache.slots.push(ZlibCacheSlot::new(pool)));
        }

        cache
    }

    fn select_slot_for_reuse(&self) -> Result<usize, ()> {
        if self.slots.capacity() > 0 {
            let mut selected = 0;
            for (i, slot) in self.slots.iter().enumerate() {
                if slot.dc.is_none() {
                    selected = i;
                    break;
                }
            }
            Ok(selected)
        } else {
            Err(())
        }
    }

    pub fn uncompress(
        &mut self,
        zdata: &'static [u8],
        offset: usize,
        dest_buf: &mut [u8],
    ) -> Result<bool, ()> {
        let slot = self
            .slots
            .iter_mut()
            .find(|slot| slot.is_for(zdata, offset));

        if let Some(slot) = slot {
            slot.uncompress(dest_buf)
        } else {
            let selected = self.select_slot_for_reuse()?;
            let slot = &mut self.slots[selected];
            slot.reset(zdata);
            slot.skip(offset)?;
            slot.uncompress(dest_buf)
        }
    }
}

impl<'a> UzlibContext<'a> {
    // TODO: move to trezorhal !!!

    pub fn skip(&mut self, nbytes: usize) -> Result<bool, ()> {
        let mut result = false; // false => OK, true => DONE
        let mut sink = [0u8; 256];
        for i in (0..nbytes).step_by(sink.len()) {
            let chunk_len = core::cmp::min(sink.len(), nbytes - i);
            let chunk = &mut sink[0..chunk_len];
            result = self.uncompress(chunk)?;
        }
        Ok(result)
    }
}
