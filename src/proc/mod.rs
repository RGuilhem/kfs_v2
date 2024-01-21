use crate::println;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;

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
    // State
    // Control
    status: ProcessStatus,
}

/// TODO: is this needed?
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
