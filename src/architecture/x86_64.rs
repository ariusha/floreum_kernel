use crate::architecture::Abi;
use ostd::arch::cpu::context::UserContext;
pub enum RegisterEvent {
    Rax(usize),
    Rbx(usize),
    Rcx(usize),
    Rdx(usize),
    Rsi(usize),
    Rdi(usize),
    Rsp(usize),
    Rbp(usize),
    R8(usize),
    R9(usize),
    R10(usize),
    R11(usize),
    R12(usize),
    R13(usize),
    R14(usize),
    R15(usize),
    Rip(usize),
    RFlags(usize),
    FsBase(usize),
    GsBase(usize),
}
impl Abi {
    pub fn read(context: &UserContext) -> Option<Self> {
        Some(Self(
            context.rax().try_into().ok()?,
            context.rdi().try_into().ok()?,
            context.rsi().try_into().ok()?,
            context.rdx().try_into().ok()?,
            context.r10().try_into().ok()?,
            context.r8().try_into().ok()?,
        ))
    }
    pub fn write(&self, context: &mut UserContext) -> Option<()> {
        context.set_rax(self.0.try_into().ok()?);
        context.set_rdi(self.1.try_into().ok()?);
        context.set_rsi(self.2.try_into().ok()?);
        context.set_rdx(self.3.try_into().ok()?);
        context.set_r10(self.4.try_into().ok()?);
        context.set_r8(self.5.try_into().ok()?);
        Some(())
    }
}
pub fn apply_event(context: &mut UserContext, event: RegisterEvent) {
        match event {
            RegisterEvent::Rax(value) => context.set_rax(value),
            RegisterEvent::Rbx(value) => context.set_rbx(value),
            RegisterEvent::Rcx(value) => context.set_rcx(value),
            RegisterEvent::Rdx(value) => context.set_rdx(value),
            RegisterEvent::Rsi(value) => context.set_rsi(value),
            RegisterEvent::Rdi(value) => context.set_rdi(value),
            RegisterEvent::Rsp(value) => context.set_rsp(value),
            RegisterEvent::Rbp(value) => context.set_rbp(value),
            RegisterEvent::R8(value) => context.set_r8(value),
            RegisterEvent::R9(value) => context.set_r9(value),
            RegisterEvent::R10(value) => context.set_r10(value),
            RegisterEvent::R11(value) => context.set_r11(value),
            RegisterEvent::R12(value) => context.set_r12(value),
            RegisterEvent::R13(value) => context.set_r13(value),
            RegisterEvent::R14(value) => context.set_r14(value),
            RegisterEvent::R15(value) => context.set_r15(value),
            RegisterEvent::Rip(value) => context.set_rip(value),
            RegisterEvent::RFlags(value) => context.set_rflags(value),
            RegisterEvent::FsBase(value) => context.set_fsbase(value),
            RegisterEvent::GsBase(value) => context.set_gsbase(value),
    }
}
