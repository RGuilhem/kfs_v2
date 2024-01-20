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

fn unknown_command() {
    println!("ERROR: unknown command");
}
