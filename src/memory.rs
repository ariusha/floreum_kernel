use crate::{
    architecture::this::{Frame, PAGE_SIZE, WRITER, flow_exit},
    language::this::{ALLOCATOR_CLAIM_MEMORY_ERR, ALLOCATOR_OUT_OF_MEMORY_NULL},
    physical::Physical,
};
use alloc::{alloc::Allocator, sync::Arc};
use core::{
    alloc::{GlobalAlloc, Layout},
    fmt::Write,
    mem::MaybeUninit,
    ptr::{self, slice_from_raw_parts},
};
use limine::memmap::{Entry, MEMMAP_USABLE};
use spinning_top::RawSpinlock;
use talc::{ErrOnOom, Span, Talc, Talck};
pub static ALLOCATOR_16: Talck<RawSpinlock, ErrOnOom> = Talck::new(Talc::new(ErrOnOom));
pub static ALLOCATOR_24: Talck<RawSpinlock, ErrOnOom> = Talck::new(Talc::new(ErrOnOom));
pub static ALLOCATOR_32: Talck<RawSpinlock, ErrOnOom> = Talck::new(Talc::new(ErrOnOom));
pub static ALLOCATOR_64: Talck<RawSpinlock, ErrOnOom> = Talck::new(Talc::new(ErrOnOom));
const ZONE_16_MAX: u64 = 1 << 16;
const ZONE_24_MAX: u64 = 1 << 24;
const ZONE_32_MAX: u64 = 1 << 32;
const ZONE_64_MAX: u64 = u64::MAX;
pub unsafe fn initialise(entries: &[&Entry]) {
    let mut alloc_16 = ALLOCATOR_16.lock();
    let mut alloc_24 = ALLOCATOR_24.lock();
    let mut alloc_32 = ALLOCATOR_32.lock();
    let mut alloc_64 = ALLOCATOR_64.lock();
    for entry in entries {
        if entry.type_ != MEMMAP_USABLE {
            continue;
        }
        let span = Span::from_base_size(
            <u64 as Into<Physical>>::into(entry.base as u64).to_virtual() as *mut u8,
            entry.length as usize,
        );
        let (span_16, span_32) = span.except(Span::from_base_size(
            ZONE_16_MAX as *mut u8,
            (ZONE_24_MAX - ZONE_16_MAX) as usize,
        ));
        let (span_24, span_64) = span.except(Span::from_base_size(
            ZONE_32_MAX as *mut u8,
            (ZONE_64_MAX - ZONE_32_MAX) as usize,
        ));
        unsafe {
            if span_16.is_sized() {
                alloc_16.claim(span_16).expect(ALLOCATOR_CLAIM_MEMORY_ERR);
            }
            if span_24.is_sized() {
                alloc_24.claim(span_24).expect(ALLOCATOR_CLAIM_MEMORY_ERR);
            }
            if span_32.is_sized() {
                alloc_32.claim(span_32).expect(ALLOCATOR_CLAIM_MEMORY_ERR);
            }
            if span_64.is_sized() {
                alloc_64.claim(span_64).expect(ALLOCATOR_CLAIM_MEMORY_ERR);
            }
        }
    }
}
pub struct ZoneGlobalAllocator;
#[global_allocator]
pub static GLOBAL_ALLOCATOR: ZoneGlobalAllocator = ZoneGlobalAllocator;
unsafe impl GlobalAlloc for ZoneGlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { ALLOCATOR_64.alloc(layout) };
        if !pointer.is_null() {
            return pointer;
        }
        let pointer = unsafe { ALLOCATOR_32.alloc(layout) };
        if !pointer.is_null() {
            return pointer;
        }
        let pointer = unsafe { ALLOCATOR_24.alloc(layout) };
        if !pointer.is_null() {
            return pointer;
        }
        let pointer = unsafe { ALLOCATOR_16.alloc(layout) };
        if !pointer.is_null() {
            return pointer;
        }
        let _ = WRITER.lock().write_str(ALLOCATOR_OUT_OF_MEMORY_NULL);
        flow_exit()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let ptr_u64 = ptr as u64;
        unsafe {
            if ptr_u64 < ZONE_16_MAX {
                &ALLOCATOR_16
            } else if ptr_u64 < ZONE_24_MAX {
                &ALLOCATOR_24
            } else if ptr_u64 < ZONE_32_MAX {
                &ALLOCATOR_32
            } else {
                &ALLOCATOR_64
            }
            .dealloc(ptr, layout)
        };
    }
}
#[derive(Clone, Copy)]
pub enum Zone {
    Addressable16,
    Addressable24,
    Addressable32,
    Addressable64,
}
pub fn contiguous_segment(frames: usize, zone: Zone) -> Arc<[Frame]> {
    unsafe {
        // todo!() allow fallback into more exclusive zones
        Arc::from_raw(
            Arc::into_raw_with_allocator(
                Arc::<[Frame], &Talck<RawSpinlock, ErrOnOom>>::new_zeroed_slice_in(
                    frames * PAGE_SIZE,
                    match zone {
                        Zone::Addressable16 => &ALLOCATOR_16,
                        Zone::Addressable24 => &ALLOCATOR_24,
                        Zone::Addressable32 => &ALLOCATOR_32,
                        Zone::Addressable64 => &ALLOCATOR_64,
                    },
                ),
            )
            .0 as *const [Frame],
        )
    }
}
