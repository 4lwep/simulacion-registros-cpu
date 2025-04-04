use std::collections::HashSet;
use std::io;

use register::Register;
use register::CPSR;
use parser::Instruction;
use crate::operations::arithmetic::*;
use crate::operations::movement::*;

mod operations;
mod parser;
mod register;

fn main() {
    let mut register_bank: HashSet<Register> = HashSet::new();

    let r0: Register = Register::new("r0".to_string());
    let r1: Register = Register::new("r1".to_string());
    let r2: Register = Register::new("r2".to_string());
    let r3: Register = Register::new("r3".to_string());
    let r4: Register = Register::new("r4".to_string());
    let r5: Register = Register::new("r5".to_string());
    let r6: Register = Register::new("r6".to_string());
    let r7: Register = Register::new("r7".to_string());
    let r8: Register = Register::new("r8".to_string());
    let r9: Register = Register::new("r9".to_string());
    let r10: Register = Register::new("r10".to_string());
    let r11: Register = Register::new("r11".to_string());
    let r12: Register = Register::new("r12".to_string());
    let r13: Register = Register::new("r13".to_string());
    let r14: Register = Register::new("r14".to_string());
    let r15: Register = Register::new("r15".to_string());
    
    let mut cpsr: CPSR = CPSR::new();

    register_bank.insert(r0);
    register_bank.insert(r1);
    register_bank.insert(r2);
    register_bank.insert(r3);
    register_bank.insert(r4);
    register_bank.insert(r5);
    register_bank.insert(r6);
    register_bank.insert(r7);
    register_bank.insert(r8);
    register_bank.insert(r9);
    register_bank.insert(r10);
    register_bank.insert(r11);
    register_bank.insert(r12);
    register_bank.insert(r13);
    register_bank.insert(r14);
    register_bank.insert(r15);

    loop{
        let mut instruction = String::new();
        let reader = io::stdin();
        reader.read_line(&mut instruction).unwrap();
        let parsed_instruction = Instruction::parse(instruction.trim().to_string(), &register_bank);
        

        let mut rd = Register::new(parsed_instruction.rd_name);
        match &parsed_instruction.operation as &str{
            "mov" => {
                mov(&mut rd, &parsed_instruction.operand2);
            }
            _ => {println!("ERROR: That operation does not exist");}
        }
        cpsr.set_flags(&rd);
        register_bank.replace(rd);
        
        register_bank.iter().for_each(|r| {
            println!("{}", r.to_string());
        });
        println!("{}", cpsr.flags_to_string());
    }

    
}