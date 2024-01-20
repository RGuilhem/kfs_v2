use crate::print;
use crate::println;
use alloc::string::String;

pub fn debug_command(command: &String) {
    println!("DEBUG: {}", command);
    print!("> ");
}
