use core::arch::asm;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::paging::OffsetPageTable;

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
    // 8 bits Not used??
}

impl Registers {
    #[inline]
    pub unsafe fn save(&mut self) {
        asm!("mov {0}, rax", out(reg) self.rax);
        asm!("mov rax, rbx", out("rax") self.rbx);
        asm!("mov rax, rcx", out("rax") self.rcx);
        asm!("mov rax, rdx", out("rax") self.rdx);
        asm!("mov {0:e}, eax", out(reg) self.eax);
        asm!("mov eax, ebx", out("eax") self.ebx);
        asm!("mov eax, ecx", out("eax") self.ecx);
        asm!("mov eax, edx", out("eax") self.edx);
        asm!("mov {0:x}, ax", out(reg) self.ax);
        asm!("mov ax, bx", out("ax") self.bx);
        asm!("mov ax, cx", out("ax") self.cx);
        asm!("mov ax, dx", out("ax") self.dx);
    }
}
