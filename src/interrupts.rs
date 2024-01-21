use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;

use crate::gdt;
use crate::hlt_loop;
use crate::println;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub const SOFTWARE_OFFSET: u8 = 0x80;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    Software = SOFTWARE_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_IDX);
        }
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt[InterruptIndex::Software.as_usize()].set_handler_fn(sowftware_interrupt_handler);
        idt
    };
}

pub fn init() {
    println!("Start of interrupts::init");
    IDT.load();
    println!("End of interrupts::init");
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    println!("Exception: DIVIDE\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("Exception: BREAKPOINT\n{:#?}", stack_frame);
    //hlt_loop(); TODO: a handler to stop and wait would be cool
    //x86_64::instructions::interrupts::enable_and_hlt();
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error: u64) -> ! {
    //_error is always 0 so we don't print it
    panic!("Exception: DOUBLE_FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    //print!(".");
    crate::vga_buffer::toggle_dot();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    };
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    };
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("Exception: PAGE_FAULT:");
    println!("At address: {:?}", Cr2::read());
    println!("error_code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop(); //TODO handle the fault
}

extern "x86-interrupt" fn sowftware_interrupt_handler(stack_frame: InterruptStackFrame) {
    println!("INTERRUPT: Software:");
    println!("{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_handler() {
    x86_64::instructions::interrupts::int3();
}

#[test_case]
fn test_timer_handler() {
    //unimplemented!();
    //x86_64::instructions::interrupts::
}
