pub mod process;
use core::sync::atomic::{AtomicBool, Ordering};
use alloc::{sync::{Arc, Weak}, vec::Vec};
use crossbeam::queue::SegQueue;
use floreum_parser::Permit;
use ostd::sync::{RwMutex, WaitQueue};
use crate::{exchange::{Requester, Responder}, link::LinkTable, tag::TagVec};
pub struct KernelDevice {
    handler: &'static fn(Vec<u8>) -> Vec<u8>,
    pub links: RwMutex<LinkTable>,
}
pub struct UserDevice {
    waiting: SegQueue<(Arc<Responder>, Vec<u8>)>,
    working: RwMutex<TagVec<(Arc<Responder>, Option<Vec<u8>>)>>,
    waitqueue: Arc<WaitQueue>,
    dropping: AtomicBool,
    pub links: RwMutex<LinkTable>,
}
impl Drop for UserDevice {
    fn drop(&mut self) {
        self.dropping.store(true, Ordering::Release);
        self.waitqueue.wake_all();
    }
}
pub enum Device {
    Kernel(KernelDevice),
    User(UserDevice),
}
impl KernelDevice {
    pub fn new(handler: &'static fn(Vec<u8>) -> Vec<u8>) -> Arc<Device> {
        Arc::new(Device::Kernel(Self {
            handler,
            links: RwMutex::new(LinkTable::new()),
        }))
    }
}
impl UserDevice {
    pub fn new() -> Arc<Device> {
        Arc::new(Device::User(Self {
            waiting: SegQueue::new(),
            working: RwMutex::new(TagVec::new()),
            waitqueue: Arc::new(WaitQueue::new()),
            dropping: AtomicBool::new(false),
            links: RwMutex::new(LinkTable::new()),
        }))
    }
}
impl Device {
    pub fn request(&self, request: Vec<u8>) -> Arc<Requester> {
        match self {
            Device::Kernel(device) => Requester::new_immediate((device.handler)(request)),
            Device::User(device) => {
                let (requester, responder) = Requester::new_pair();
                device.waiting.push((responder, request));
                device.waitqueue.wake_one();
                requester
            },
        }
    }
    pub fn respond(&self, request: u64, response: Vec<u8>) -> Option<()> {
        if let Device::User(user) = self {
        let working_upread = user.working.upread();
        if working_upread.get(request).is_some() {
            working_upread.upgrade().remove(request).map(|(responder, _)| responder.respond(response))
        } else {
            None
        }
        } else {
            None
        }
    }
    pub fn block(self: Arc<Self>) -> Option<u64> {
        if let Device::User(user) = self.as_ref() {
        let waitqueue = user.waitqueue.clone();
        let weak = Arc::downgrade(&self);
        drop(self);
        let ((responder, request), device) = waitqueue.wait_until(|| match weak.upgrade() {
            Some(new_self) => if let Device::User(new_user) = new_self.as_ref() {
                new_user.waiting.pop().map(|request| Some((request, new_self)))
            } else {
                unreachable!()
            },
            None => Some(None),
        })?;
        if let Device::User(user) = device.as_ref() {
            Some(user.working.write().push((responder, Some(request))))
        } else {
            unreachable!()
        }
        } else {
            None
        }
    }
    pub fn links(&self) -> LinkTable {
        match self {
            Device::Kernel(device) => device.links.read().clone(),
            Device::User(device) => device.links.read().clone(),
        }
    }
}