use core::sync::atomic::{AtomicBool, Ordering};
use crate::{event::Event, user::user_task};
use alloc::{sync::{Arc, Weak}, vec::Vec};
use crossbeam::queue::SegQueue;
use ostd::{arch::cpu::context::{CpuException, UserContext}, mm::VmSpace, sync::{RwLock, RwMutex, Waiter, Waker}, task::{Task, TaskOptions}};
use spin::Once;
pub struct Process {
    parent: Option<Weak<Process>>,
    pub memory: Arc<VmSpace>,
    pub threads: RwMutex<Vec<Arc<Thread>>>,
    pub children: RwMutex<Vec<Arc<Process>>>,
}
impl Process {
    pub fn parent(&self) -> Option<Weak<Process>> {
        self.parent.clone()
    }
    pub fn add_thread(self: &Arc<Self>, context: UserContext) -> Option<Arc<Thread>> {
        let thread = Arc::new(Thread {
            process: Arc::downgrade(self),
            parked: AtomicBool::new(false),
            park_waker: Once::new(),
            context: RwLock::new((context, None)),
            events: SegQueue::new(),
        });
        let weak = Arc::downgrade(&thread);
        let task = Arc::new(TaskOptions::new(move || unsafe { user_task(weak.clone()); }).build().ok()?);
        self.threads.write().push(thread.clone());
        task.run();
        Some(thread)
    }
    pub fn add_child(self: &Arc<Self>) -> Arc<Self> {
        let child = Arc::new(Self {
            parent: Some(Arc::downgrade(self)),
            memory: Arc::new(VmSpace::new()),
            threads: RwMutex::new(Vec::new()),
            children: RwMutex::new(Vec::new()),
        });
        self.children.write().push(child.clone());
        child
    }
    pub fn kickstart() -> Arc<Self> {
        Arc::new(Self {
            parent: None,
            memory: Arc::new(VmSpace::new()),
            threads: RwMutex::new(Vec::new()),
            children: RwMutex::new(Vec::new()),
        })
    }
}
pub struct Thread {
    process: Weak<Process>,
    parked: AtomicBool,
    pub park_waker: Once<Arc<Waker>>,
    pub context: RwLock<(UserContext, Option<CpuException>)>,
    pub events: SegQueue<Event>,
}
impl Thread {
    pub fn process(&self) -> Weak<Process> {
        self.process.clone()
    }
    pub fn parked(&self) -> bool {
        self.parked.load(Ordering::Relaxed)
    }
    pub fn park(&self) {
        self.events.push(Event::Park);
    }
    pub fn unpark(&self) -> bool {
        !self.park_waker.wait().wake_up()
    }
    pub unsafe fn mark_parked(&self, value: bool) {
        self.parked.store(value, Ordering::Relaxed);
    }
}