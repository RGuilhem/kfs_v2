use x86_64::structures::paging::OffsetPageTable;
use x86_64::structures::idt::InterruptStackFrame;

pub struct ProcessContext {
    page_table: OffsetPageTable<'static>,
    stack_frame: InterruptStackFrame,
    registers: Registers,
}

impl ProcessContext {
    pub fn new(page_table: OffsetPageTable<'static>, stack_frame: InterruptStackFrame) -> Self {
        Self {
            page_table,
            stack_frame,
            registers: Registers::default(),
        }
    }
}

#[derive(Default)]
pub struct Registers {
    // 64 bits
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    // 32 bits
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    // 16 bits
    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,
    // 8 bits
    ah: u8,
    al: u8,
    bh: u8,
    bl: u8,
    ch: u8,
    cl: u8,
    dh: u8,
    dl: u8,
}

impl Registers {
}
