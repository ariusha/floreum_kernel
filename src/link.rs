use crate::device::Device;
use alloc::{
    string::String,
    sync::{Arc, Weak},
    vec::Vec,
};
use floreum_parser::Permit;
#[derive(Clone)]
pub struct Link {
    device: Weak<Device>,
    from: Arc<str>,
    to: Arc<str>,
    permit: Permit,
}
#[derive(Clone)]
pub struct LinkTable(Arc<[Link]>);
impl LinkTable {
    pub fn new() -> Self {
        Self(Vec::new().into())
    }
    pub fn link(&mut self, device: Weak<Device>, from: Arc<str>, to: Arc<str>, permit: Permit) {
        let mut as_vec = self.0.to_vec();
        as_vec.retain(|link| from.strip_prefix(link.from.as_ref()).is_none());
        as_vec.push(Link {
            device,
            from,
            to,
            permit,
        });
        self.0 = as_vec.into()
    }
    pub fn resolve(&self, device: Weak<Device>, permit: Permit, path: &str) -> (Weak<Device>, Permit, String) {
        self
            .0
            .iter()
            .filter_map(|link| Some((link, path.strip_prefix(link.from.as_ref())?)))
            .filter(|(_, relative)| relative.is_empty() | relative.starts_with('/'))
            .max_by_key(|(_, relative)| relative.len())
            .map(|(link, relative)| Some(link.device.upgrade()?.links().resolve(link.device.clone(), permit & link.permit, relative)))
            .flatten()
            .unwrap_or((device, permit, path.into()))
    }
}
