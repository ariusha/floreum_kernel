use crate::{process::Process, thread::Thread};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::sync::atomic::AtomicBool;
use crossbeam::queue::SegQueue;
use elf::{ElfBytes, abi::PT_LOAD, endian::LittleEndian};
use ostd::{
    arch::cpu::context::UserContext,
    mm::{CachePolicy, FrameAllocOptions, PAGE_SIZE, PageFlags, PageProperty, VmIo, VmSpace},
    sync::{RwLock, RwMutex},
    task::disable_preempt,
    user::UserContextApi,
};
pub fn new(elf_bytes: &[u8]) -> &'static Arc<Process> {
    let elf: ElfBytes<'_, LittleEndian> = elf::ElfBytes::minimal_parse(elf_bytes).unwrap();
    let memory = Arc::new(VmSpace::new());
    memory.activate();
    let guard = disable_preempt();
    for header in elf
        .segments()
        .unwrap()
        .iter()
        .filter(|header| header.p_type == PT_LOAD)
    {
        let segment_length: usize = header.p_memsz.try_into().unwrap();
        let segment = FrameAllocOptions::new()
            .alloc_segment((segment_length + PAGE_SIZE - 1) / PAGE_SIZE)
            .unwrap();
        let segment_offset = header.p_offset.try_into().unwrap();
        segment
            .write_bytes(
                0,
                elf_bytes
                    .get(segment_offset..segment_offset + segment_length)
                    .unwrap(),
            )
            .unwrap();
        let header_virtual = header.p_vaddr.try_into().unwrap();
        let mut cursor = memory
            .cursor_mut(
                &guard,
                &(header_virtual
                    ..((header_virtual + segment_length + PAGE_SIZE - 1) / PAGE_SIZE) * PAGE_SIZE),
            )
            .unwrap();
        let properties = PageProperty::new_user(
            if header.p_flags & elf::abi::PF_R != 0 {
                PageFlags::R
            } else {
                PageFlags::empty()
            } | if header.p_flags & elf::abi::PF_W != 0 {
                PageFlags::W
            } else {
                PageFlags::empty()
            } | if header.p_flags & elf::abi::PF_X != 0 {
                PageFlags::X
            } else {
                PageFlags::empty()
            },
            CachePolicy::Writethrough,
        );
        for frame in segment {
            cursor.map(frame.into(), properties);
        }
    }
    let process = Arc::new(Process {
        memory,
        threads: RwMutex::new(Vec::new()),
    });
    let mut context = UserContext::default();
    context.set_instruction_pointer(elf.ehdr.e_entry.try_into().unwrap());
    let symbol_table = elf.symbol_table().unwrap().unwrap();
    context.set_stack_pointer(
        symbol_table
            .0
            .iter()
            .find(|symbol| {
                symbol_table
                    .1
                    .get(symbol.st_name.try_into().unwrap())
                    .unwrap()
                    == ".stack"
            })
            .unwrap()
            .st_value
            .try_into()
            .unwrap(),
    );
    // context.set_tls_pointer(todo!());
    let thread = Arc::new(Thread {
        process: Arc::downgrade(&process),
        executing: AtomicBool::new(true),
        context: RwLock::new(context),
        events: SegQueue::new(),
        exception: RwLock::new(None),
    });
    (*process.threads.write()).push(thread);
    Box::leak::<'static>(Box::new(process))
}
