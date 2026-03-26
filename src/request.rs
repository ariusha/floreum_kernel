use alloc::{format, rc::Weak, string::String, vec::Vec};
use floreum_parser::{Entry, Message, Permit, Request, Response};
use crate::{device::Device, process::Process};
use ostd::sync::RwMutex;
use core::sync::atomic::AtomicBool;
use crossbeam::queue::SegQueue;
pub fn handle(process: &Process, request: Vec<u8>) -> Option<u64> {
    match <Message<&str, &[u8], Vec<Entry<&str>>> as TryInto<Request<&str, &[u8]>>>::try_into(Message::from_bytes(request.as_slice()).ok()?).ok()? {
        Request::Open(request) => {
            let path = process.workspace.read().clone().resolve(process.device(), todo!(), format!("/workspace/{}", request.path));
            todo!()
        },
        Request::Flush(request) => todo!(),
        Request::Close(request) => todo!(),
        Request::Metadata(request) => todo!(),
        Request::Setmeta(request) => todo!(),
        Request::List(request) => todo!(),
        Request::Remove(request) => todo!(),
        Request::Read(request) => todo!(),
        Request::Write(request) => todo!(),
        Request::Seek(request) => todo!(),
        Request::Copy(request) => todo!(),
        Request::Link(request) => todo!(),
        Request::Drop(request) => todo!(),
    }
}