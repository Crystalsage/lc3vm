use std::env;
use std::process::exit;
use std::fs;

mod types;
use types::{VM, Opcode};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("You need to pass the program itself");
        exit(1);
    }

    let program_name = args[1].clone();
    let program: Vec<u8>  = read_program(program_name);

    // Setup
    let mut vm: VM = VM::init();

    loop {
        // Fetch
        vm.pc += 1;
        let instr: u16 = todo!();
        let op: Opcode = (instr >> 12).try_into().unwrap();

        match op.into() {
            Opcode::ADD => {
            }
            Opcode::BR => todo!(),
            Opcode::LD => todo!(),
            Opcode::ST => todo!(),
            Opcode::JSR => todo!(),
            Opcode::AND => todo!(),
            Opcode::LDR => todo!(),
            Opcode::STR => todo!(),
            Opcode::RTI => todo!(),
            Opcode::NOT => todo!(),
            Opcode::LDI => todo!(),
            Opcode::STI => todo!(),
            Opcode::JMP => todo!(),
            Opcode::RES => todo!(),
            Opcode::LEA => todo!(),
            Opcode::TRAP => todo!(),
        }
        // Execute
        break;
    }

    println!("[+] Cleaning up VM");
    VM::cleanup(&mut vm);

    println!("[+] Finishing");
}

fn read_program(program_name: String) -> Vec<u8> {
    return fs::read(program_name).unwrap();
}
