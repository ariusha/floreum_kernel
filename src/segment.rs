use core::iter::once;

use crate::{
    architecture::this::{ArchitectureSegment, CONTIGUOUS_COUNT, Frame},
    memory::{Zone, contiguous_segment},
};
use alloc::{sync::Arc, vec::Vec};
#[derive(Clone)]
pub enum SegmentState {
    Owned,
    Shared,
    Cow,
}
#[derive(Clone)]
pub struct Segment {
    pub architecture: ArchitectureSegment,
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub physical: Vec<Arc<[Frame]>>,
    state: SegmentState,
}
impl Segment {
    pub fn new(
        frames: usize,
        architecture: ArchitectureSegment,
        zone: Zone,
        read: bool,
        write: bool,
        execute: bool,
    ) -> Self {
        let contiguous_frames = frames / CONTIGUOUS_COUNT;
        Self {
            architecture,
            read,
            write,
            execute,
            physical: (0..contiguous_frames)
                .map(|_| contiguous_segment(CONTIGUOUS_COUNT, zone))
                .chain(once(contiguous_segment(
                    frames - contiguous_frames * CONTIGUOUS_COUNT,
                    zone,
                )))
                .collect(),
            state: SegmentState::Owned,
        }
    }
    pub fn clone_cow(&mut self, restrict: ArchitectureSegment) -> Self {
        if matches!(self.state, SegmentState::Shared) {
            self.physical = self.physical.to_vec().into(); // todo!() avoid cloning if strong count == 1.
            self.state = SegmentState::Cow;
        }
        Self {
            architecture: self.architecture,
            read: self.read,
            write: self.write,
            execute: self.execute,
            physical: self.physical.clone(),
            state: SegmentState::Cow,
        }
    }
    pub fn clone_shared(&mut self, restrict: ArchitectureSegment) -> Self {
        match self.state {
            SegmentState::Owned => self.state = SegmentState::Shared,
            SegmentState::Shared => {}
            SegmentState::Cow => self.physical = self.physical.to_vec().into(),
        }
        Self {
            architecture: self.architecture.clone(),
            read: self.read,
            write: self.write,
            execute: self.execute,
            physical: self.physical.clone(),
            state: SegmentState::Cow,
        }
    }
    pub fn restrict(mut self, read: bool, write: bool, execute: bool) -> Self {
        self.read &= read;
        self.write &= write;
        self.execute &= execute;
        self
    }
}
