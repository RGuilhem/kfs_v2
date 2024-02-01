use crate::println;
use core::arch::asm;

#[inline(always)]
pub fn dispatch_syscall(code: usize) {
    println!("code: {:#x}", code);
    println!("end of dispatch_syscall")
}

#[inline]
pub fn do_syscall(code: usize) {
    unsafe {
        asm!("push {}", in(reg) code);
        asm!("int {}", const 0x80, options(nomem));
        asm!("pop rax")
        //asm!("int {id}", id = const 0x80, options(nomem, nostack));
    };
    println!("Before end do_syscall");
}
