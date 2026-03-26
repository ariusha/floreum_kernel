use alloc::{rc::Weak, string::String, vec::Vec};
use floreum_parser::{Entry, Message, Permit, Request, Response};
use crate::{device::{Device, Link}, process::Process};
pub fn resolve(links: &[Link], permit: Permit, path: &'a str) -> (Weak<Device>, Permit, String) {
    
}
pub fn handle(process: &Process, request: Vec<u8>) -> Option<u64> {
    match <Message<&str, &[u8], Vec<Entry<&str>>> as TryInto<Request<&str, &[u8], Vec<Entry<&str>>>>>::try_into(Message::from_bytes(request.as_slice()).ok()?).ok()? {
        Request::Open(request_open) => ,
        Request::Flush(request_flush) => todo!(),
        Request::Close(request_close) => todo!(),
        Request::Metadata(request_metadata) => todo!(),
        Request::Setmeta(request_setmeta) => todo!(),
        Request::List(request_list) => todo!(),
        Request::Remove(request_remove) => todo!(),
        Request::Read(request_read) => todo!(),
        Request::Write(request_write) => todo!(),
        Request::Seek(request_seek) => todo!(),
        Request::Copy(request_copy) => todo!(),
        Request::Link(request_link) => todo!(),
        Request::Drop(request_drop) => todo!(),
    }
}