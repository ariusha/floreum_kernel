use crate::{
    architecture::this::PAGE_SIZE,
    language::this::{
        FRAME_ALLOCATOR_DEALLOCATE_INVALID_ORDER, FRAME_ALLOCATOR_DOUBLE_MERGE_ERR, FRAME_ALLOCATOR_HIGHER_SPLIT_ERR, FRAME_ALLOCATOR_INVALID_INDEX, FRAME_ALLOCATOR_NOT_INITIALISED, LIMINE_MEMORY_MAP_ALLOCATOR_LARGE, LIMINE_MEMORY_MAP_ENTRIES_NONE, LIMINE_MEMORY_MAP_RESPONSE_NONE
    }, physical::Physical,
};
use core::{
    array, mem::ManuallyDrop, ops::Range, ptr::slice_from_raw_parts, sync::atomic::{AtomicUsize, Ordering}
};
use limine::{
    memmap::{Entry, MEMMAP_USABLE},
    request::MemmapRequest,
};
use spin::Once;
struct Zone {
    frames: Range<usize>,
    hint: AtomicUsize,
}
impl Zone {
    pub const fn new(frames: Range<usize>) -> Self {
        Self {
            frames: frames.start..frames.end,
            hint: AtomicUsize::new((frames.end - 1) % usize::BITS as usize),
        }
    }
}
const MAX_ORDERS: usize = (u64::BITS - PAGE_SIZE.ilog2()) as usize;
static ORDERS: Once<[&'static [AtomicUsize]; MAX_ORDERS]> = Once::new();
pub struct Frame {
    order: usize,
    index: usize, // this is the index within this order: if there are 8 frames IN TOTAL, and this frame's order=1 (it is 2 frames glued together) and this frame's index=2, then it covers the actual frames 4+5, not 2+3.
}
impl Frame {
    pub fn merge(&mut self, sibling: Frame) -> Result<(), Self> {
        if self.order == sibling.order
            && ((self.index == sibling.index + 1) || (self.index + 1 == sibling.index))
        {
            let _ = ManuallyDrop::new(sibling);
            self.order += 1;
            self.index /= 2;
            Ok(())
        } else {
            Err(sibling)
        }
    }
    pub fn split(&mut self) -> Option<Self> {
        // any use of .split() does cleanly split the frame (if possible), such that it is correct to let the destructor run if the sibling is not needed
        if self.order == 0 {
            None
        } else {
            self.order -= 1;
            self.index *= 2;
            Some(Frame {
                order: self.order,
                index: self.index + 1,
            })
        }
    }
    pub fn double(&mut self) -> Result<(), ()> {
        Ok(self
            .merge(Self::new_specific(self.order, self.index ^ (1 << self.order)).ok_or(())?) // fixed. thank you.
            .map_err(|_| ())
            .expect(FRAME_ALLOCATOR_DOUBLE_MERGE_ERR)) // failure to allocate with new_specific exits early with a shortcut, but if it succeeds, then merging should always succeed, so we panic on failure because an invariant has been broken somewhere
    }
    pub fn new_specific(order: usize, index: usize) -> Option<Self> {
        let usize_bits = usize::BITS as usize;
        let bit = 1 << (index % usize_bits);
        if ORDERS
            .get()
            .expect(FRAME_ALLOCATOR_NOT_INITIALISED)
            .get(order)
            .expect(FRAME_ALLOCATOR_INVALID_INDEX)
            .get(index / usize_bits)?
            .fetch_and(!bit, Ordering::Acquire)
            & bit
            == 0
        {
            None
        } else {
            Some(Frame { order, index })
        }
    }
    pub fn new(order: usize, zone: &'static Zone) -> Option<Self> {
        let usize_bits = usize::BITS as usize;
        let bitmap = ORDERS
            .get()
            .expect(FRAME_ALLOCATOR_NOT_INITIALISED)
            .get(order)?;
        let try_usize = |usize_index| {
            bitmap
                .get(usize_index)
                .map(|atomic: &AtomicUsize| {
                    for bit in 0..usize_bits {
                        if atomic.load(Ordering::Acquire) & (1 << bit) != 0 {
                            if let Some(successful) =
                                Self::new_specific(order, usize_index * usize_bits + bit)
                            // fixed. thanks.
                            {
                                zone.hint.store(
                                    (usize_index * usize_bits) << order, // now stores the index of the first order-0 frame from the most recent allocation
                                    Ordering::Release, // uses release ordering for the hint, to allow synchronisation on weak-ordering architectures
                                );
                                return Some(successful);
                            }
                        }
                    }
                    None
                })
                .flatten()
        };
        let hint = (zone.hint.load(Ordering::Acquire) >> order) * usize_bits;
        for usize_index in ((zone.frames.start / usize_bits).max(zone.frames.start)
            ..hint.min(zone.frames.end))
            .into_iter()
            .rev()
        {
            if let Some(success) = try_usize(usize_index) {
                return Some(success);
            }
        }
        for usize_index in (hint.max(zone.frames.start)
            ..((zone.frames.end + usize_bits - 1) / usize_bits).min(zone.frames.end))
            .into_iter()
            .rev()
        {
            if let Some(success) = try_usize(usize_index) {
                return Some(success);
            }
        }
        let mut higher = Self::new(order + 1, zone)?; // i also noticed this should have been order+1, not -1
        higher.split().expect(FRAME_ALLOCATOR_HIGHER_SPLIT_ERR); // sibling automatically dropped, which handles deallocation in the correct bitmap. this should never fail, because it has been allocated from a non-zero-order bitmap, so we panic because an invariant has been broken.
        Some(higher)
    }
}
impl Drop for Frame {
    fn drop(&mut self) {
        let usize_bits = usize::BITS as usize;
        while let Ok(_) = self.double() {}
        ORDERS
            .get()
            .expect(FRAME_ALLOCATOR_NOT_INITIALISED)
            .get(self.order)
            .expect(FRAME_ALLOCATOR_DEALLOCATE_INVALID_ORDER)
            .get(self.index / usize_bits)
            .unwrap()
            .fetch_or(1 << (self.index % usize_bits), Ordering::Release);
    }
}
#[used]
#[unsafe(link_section = ".requests")]
pub static MEMORY_MAP: MemmapRequest = MemmapRequest::new();
pub unsafe fn initialise() {
    let entries = MEMORY_MAP
        .response()
        .expect(LIMINE_MEMORY_MAP_RESPONSE_NONE)
        .entries();
    let frames = (entries
        .iter()
        .filter(|entry| entry.type_ == MEMMAP_USABLE)
        .map(|entry| entry.base + entry.length)
        .max()
        .expect(LIMINE_MEMORY_MAP_ENTRIES_NONE)
        - 1)
        / PAGE_SIZE;
    let metadata_bytes = frames / (usize::BITS as u64) * size_of::<AtomicUsize>() as u64;
    let metadata_region = entries.iter().filter(|entry| entry.length >= metadata_bytes).next().expect(LIMINE_MEMORY_MAP_ALLOCATOR_LARGE);
    let array: &'static [AtomicUsize] = unsafe {&*slice_from_raw_parts(Physical::from(metadata_region.base).to_virtual() as *const AtomicUsize, metadata_bytes as usize)};
    for atomic in array {
        atomic.store(0, Ordering::Release);
    }
    let orders = array::from_fn(|index| &array[0..0]);
    ORDERS.call_once(|| orders);
}
