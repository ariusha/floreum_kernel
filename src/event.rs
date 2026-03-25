use crate::architecture::this::RegisterEvent;
pub enum Event {
    Register(RegisterEvent),
    Park,
}