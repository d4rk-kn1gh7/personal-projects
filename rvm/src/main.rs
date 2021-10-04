#[macro_use] extern crate text_io;
use std::env;
use std::process;
pub mod vm;
pub mod instruction;
pub mod assembler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Enter path to asm file as argv[1]!");
        process::exit(1);
    }
    let mut vm1 = vm::VM::new();
    let mut asm1 = assembler::Asm::new();
    vm1.instructions = asm1.parse_to_bytecode(&args[1]);
    vm1.run();
    vm1.print_reg_state();
}