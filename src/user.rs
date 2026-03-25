use crate::{architecture::{Abi, this::apply_event}, event::Event, handle::Handle, process::Thread, syscall};
use alloc::sync::Weak;
use ostd::{sync::Waiter, task::disable_preempt, user::{ReturnReason, UserMode}};
pub unsafe fn user_task(thread: Weak<Thread>) -> Option<()> {
    let preempt_guard = disable_preempt();
    let (park_waiter, park_waker) = Waiter::new_pair();
    thread.upgrade()?.park_waker.call_once(|| park_waker);
    drop(preempt_guard);
    let handle = Handle::new(&thread, &park_waiter);
    let mut user_mode = UserMode::new(thread.upgrade()?.context.read().0.clone());
    loop {
        let reason = user_mode.execute(|| match thread.upgrade() {
            Some(thread_arc) => thread_arc.events.len() != 0,
            None => true,
        });
        let exception = user_mode.context_mut().take_exception();
        *thread.upgrade()?.context.write() = (user_mode.context().clone(), exception);
        match reason {
            ReturnReason::UserSyscall => {
                syscall::handle(&handle, Abi::read(user_mode.context())?)?
                    .write(user_mode.context_mut())?
            },
            ReturnReason::UserException => handle.park()?,
            ReturnReason::KernelEvent => {
                let thread_arc = thread.upgrade()?;
                while let Some(event) = thread_arc.events.pop() {
                match event {
                        Event::Register(register_event) => apply_event(user_mode.context_mut(), register_event),
                        Event::Park => handle.park()?,
                    }
                }
                
            },
        }
    }
}
