use alloc::{sync::{Arc, Weak}, vec::Vec};
use ostd::sync::{RwMutex, WaitQueue};
pub struct Requester {
    response: RwMutex<Option<Vec<u8>>>,
    responder: Weak<Responder>,
    waitqueue: Arc<WaitQueue>,
}
pub struct Responder {
    requester: Weak<Requester>,
}
impl Drop for Requester {
    fn drop(&mut self) {
        self.waitqueue.wake_all();
    }
}
impl Requester {
    pub fn new_pair() -> (Arc<Self>, Arc<Responder>) {
        let mut responder = Arc::new(Responder {
            requester: Weak::new(),
        });
        let requester = Arc::new_cyclic(|cyclic| {
            let new_responder = Arc::new(Responder {
                requester: cyclic.clone(),
            });
            responder = new_responder.clone();
            Self {
                response: RwMutex::new(None),
                responder: Arc::downgrade(&new_responder),
                waitqueue: Arc::new(WaitQueue::new()),
            }
        });
        (requester, responder)
    }
    pub fn query(&self) -> Option<Option<u64>> {
        if let Some(response) = self.response.read().as_ref() {
            Some(Some(response.len().try_into().ok()?))
        } else if self.responder.upgrade().is_some() {
            Some(None)
        } else {
            None
        }
    }
    pub fn block(self: Arc<Self>) -> Option<Vec<u8>> {
        let waitqueue = self.waitqueue.clone();
        let weak = Arc::downgrade(&self);
        drop(self);
        waitqueue.wait_until(|| {
            if let Some(this) = weak.upgrade() {
                let response_upread = this.response.upread();
                if response_upread.is_some() {
                    Some(response_upread.upgrade().take())
                } else {
                    Some(None)
                }
            } else {
                Some(None)
            }
        })
    }
}
impl Drop for Responder {
    fn drop(&mut self) {
        self.requester.upgrade().map(|requester| requester.waitqueue.wake_all());
    }
}
impl Responder {
    pub fn respond(&self, response: Vec<u8>) {
        self.requester
            .upgrade()
            .map(|requester| *requester.response.write() = Some(response));
    }
}
