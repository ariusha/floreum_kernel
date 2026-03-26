use core::sync::atomic::{AtomicBool, Ordering};
use alloc::{sync::{Arc, Weak}, vec::Vec};
use crossbeam::queue::SegQueue;
use floreum_parser::Permit;
use ostd::sync::{RwMutex, WaitQueue};
use crate::{exchange::{Requester, Responder}, tag::TagVec};
pub struct Link {
    device: Weak<Device>,
    from: Arc<str>,
    to: Arc<str>,
    permit: Permit,
}
pub struct Device {
    waiting: SegQueue<(Arc<Responder>, Vec<u8>)>,
    working: RwMutex<TagVec<(Arc<Responder>, Option<Vec<u8>>)>>,
    waitqueue: Arc<WaitQueue>,
    dropping: AtomicBool,
    pub links: RwMutex<Arc<[Link]>>,
}
impl Drop for Device {
    fn drop(&mut self) {
        self.dropping.store(true, Ordering::Release);
    }
}
impl Device {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            waiting: SegQueue::new(),
            working: RwMutex::new(TagVec::new()),
            waitqueue: Arc::new(WaitQueue::new()),
            dropping: AtomicBool::new(false),
            links: RwMutex::new(Vec::new().into()),
        })
    }
    pub fn request(&self, request: Vec<u8>) -> Arc<Requester> {
        let (requester, responder) = Requester::new_pair();
        self.waiting.push((responder, request));
        self.waitqueue.wake_one();
        requester
    }
    pub fn respond(&self, request: u64, response: Vec<u8>) -> Option<()> {
        let working_upread = self.working.upread();
        if working_upread.get(request).is_some() {
            working_upread.upgrade().remove(request).map(|(responder, _)| responder.respond(response))
        } else {
            None
        }
    }
    pub fn block(self: Arc<Self>) -> Option<u64> {
        let waitqueue = self.waitqueue.clone();
        let weak = Arc::downgrade(&self);
        drop(self);
        let ((responder, request), device) = waitqueue.wait_until(|| match weak.upgrade() {
            Some(this) => this.waiting.pop().map(|request| Some((request, this))),
            None => Some(None),
        })?;
        Some(device.working.write().push((responder, Some(request))))
    }
}