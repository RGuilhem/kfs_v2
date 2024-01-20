use crate::exit_qemu;
use crate::print;
use crate::println;
use crate::QemuExitCode;
use alloc::string::String;

pub fn debug_command(command: &String) {
    if command.len() > 0 {
        println!("DEBUG: {}", command);
        match command.as_str() {
            "help" => display_help(),
            "exit" => exit_qemu(QemuExitCode::Success),
            &_ => unknown_command(),
        }
    }
    print!("\n> ");
}

fn display_help() {
    println!("Debug commands:");
    println!("help: print this help message");
    println!("exit: exit qemu");
}

fn unknown_command() {
    println!("ERROR: unknown command");
}
