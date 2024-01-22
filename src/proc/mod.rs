//TODO: remove when done
#![allow(dead_code)]

use crate::proc::context::ProcessContext;
use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::PrivilegeLevel;

pub mod context;
/// INFO:
/// https://en.wikipedia.org/wiki/Scheduling_(computing)
/// Long term: Decides which Processes are to
/// be admitted to the ready Queue and loaded to
/// main memory
///
/// Medium term: Decides which processes should be
/// swapped out or swapped in
///
/// Short term: After clock interrupt, io iterrupt,
/// syscall or other signal. Decides which of the
/// ready in memory processes to run
///
/// Dispatcher: handles context switch, going to
/// user mode and restarting user program at
/// correct location
///
pub mod scheduler;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(u64);

impl ProcessId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        ProcessId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessStatus {
    Created,
    Ready,
    Running,
    Blocked,
    Waiting,
    Dead,
}

/// INFO:
/// See this link
/// https://en.wikipedia.org/wiki/Process_control_block
pub struct Process {
    // Identification
    id: ProcessId,
    // State: Info saved when switching process
    context: ProcessContext,
    // Control
    status: ProcessStatus,
    father: ProcessId,
    privilege: PrivilegeLevel,
    //children: ProcessId[],
}

impl Process {
    pub fn init(page_table: OffsetPageTable<'static>, stack_frame: InterruptStackFrame) -> Self {
        let id = ProcessId::new();
        Self {
            id,
            context: ProcessContext::new(page_table, stack_frame),
            father: id,
            privilege: PrivilegeLevel::Ring0,
            status: ProcessStatus::Created,
        }
    }
}
