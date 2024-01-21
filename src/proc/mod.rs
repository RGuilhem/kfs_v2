use crate::println;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;

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
    Terminated,
}

/// representation of a process
/// 
/// id
/// status
/// father
/// children
/// stack pointer and heap (paging info)
/// signals queue
/// owner id
pub struct Process {
    id: ProcessId,
    status: ProcessStatus,
}

impl Process {
    pub fn new() -> Self {
        Process {
            id: ProcessId::new(),
            status: ProcessStatus::Created,
        }
    }

    pub fn print_info(&self) {
        println!("Proc id: {:?}", self.id);
        println!("Proc status: {:?}", self.status);
    }
}
