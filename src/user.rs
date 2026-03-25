use crate::{architecture::{Abi, this::apply_event}, event::Event, syscall, thread::Thread};
use alloc::sync::Weak;
use ostd::user::UserMode;
pub unsafe fn user_task(thread: Weak<Thread>) -> Option<()> {
    let mut user_mode = UserMode::new(thread.upgrade()?.context.read().clone());
    loop {
        let reason = user_mode.execute(|| match thread.upgrade() {
            Some(thread_arc) => thread_arc.events.len() != 0,
            None => true,
        });
        user_mode.context_mut().take_exception().map(|exception| {
            *thread.upgrade()?.exception.write() = Some(exception);
            Some(())
        });
        *thread.upgrade()?.context.write() = user_mode.context().clone();
        match reason {
            ostd::user::ReturnReason::UserSyscall => {
                syscall::handle(&thread, Abi::read(user_mode.context())?)?
                    .write(user_mode.context_mut())?
            },
            ostd::user::ReturnReason::UserException => return None,
            ostd::user::ReturnReason::KernelEvent => {
                let thread_arc = thread.upgrade()?;
                while let Some(event) = thread_arc.events.pop() {
                match event {
                        Event::Register(register_event) => apply_event(user_mode.context_mut(), register_event),
                        Event::Abort => unsafe {thread_arc.self_abort()}?,
                    }
                }
                
            },
        }
    }
}
