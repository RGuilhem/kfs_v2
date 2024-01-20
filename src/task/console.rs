use crate::exit_qemu;
use crate::print;
use crate::println;
use crate::QemuExitCode;
use alloc::string::String;
use alloc::vec::Vec;

pub fn debug_command(line: &String) {
    if line.len() > 0 {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let command = split[0];
        let args = &split[1..];
        println!("DEBUG: command: {}", command);
        println!("DEBUG: args: {:?}", args);
        match command {
            "help" => help(),
            "exit" => exit(),
            "translate" => translate(args),
            "t" => translate(args),
            "regs" => regs(args),
            &_ => unknown_command(),
        }
    }
    print!("\n> ");
}

fn help() {
    println!("Debug commands:");
    println!("help: print this help message");
    println!("exit: exit qemu");
}

fn exit() {
    // TODO: handle cleanup before exit
    exit_qemu(QemuExitCode::Success);
}

fn regs(_args: &[&str]) {
    use x86_64::instructions::tables;
    use x86_64::registers::control;
    use x86_64::registers::rflags;

    println!("Cr0: {:?}", control::Cr0::read());
    println!("Cr2 PFLA: {:?}", control::Cr2::read());
    println!("Cr3: {:?}", control::Cr3::read());
    println!("Cr4: {:?}", control::Cr4::read());
    println!("{:?}", rflags::read());

    println!("\nGDT: {:?}", tables::sgdt());
    println!("IDT: {:?}", tables::sidt());
}

fn translate(_args: &[&str]) {
    use x86_64::VirtAddr;

    let _addr = VirtAddr::new(0);
    println!("Translating addresses is not implemented yet");
}

fn unknown_command() {
    println!("ERROR: unknown command");
}
