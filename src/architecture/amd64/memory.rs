use crate::{segment::Segment, tagmap::TagMap};
use core::{mem::MaybeUninit, ops::BitAnd};
use x86_64::structures::paging::PageTable;
pub const PAGE_SIZE: u64 = 4096;
#[derive(Clone, Copy)]
pub struct ArchitectureSegment {
    pub write_through: bool,
    pub no_cache: bool,
}
pub struct AddressSpace {
    table: PageTable,
    mappings: TagMap<Segment>,
}
impl AddressSpace {
    fn find(&mut self, frames: usize) -> usize {
        todo!() // find the first fit for a segment of size `frames`.
    }
    pub fn map(&mut self, segment: Segment) -> usize {
        todo!(); // map the segment at Self::find()
        self.mappings.push(segment)
    }
    pub fn map_fixed(&mut self, segment: Segment, address: usize) -> usize {
        todo!();
        self.mappings.push(segment)
    }
}
