use std::collections::HashSet;
use crate::register::Register;

pub struct Instruction{
    pub operation: String,
    pub rd: Register,
    pub rn: Option<Register>,
    pub operand2: i64
}

impl Instruction{
    pub fn parse(instruction: String, register_bank: &HashSet<Register>) -> Option<Self>{
        if !instruction.contains(','){
            return None;
        }
        
        let parts = instruction.split(',');
        let parts: Vec<&str> = parts.collect();

        let op = parts[0].trim();

        let size = match op{
            "mov" | "mvn" => 2,
            "add" | "sub" | "mul" | "rsb" | "and" | "eor" | "orr" => 3,
            _ => 0
        };

        if size == 0 || parts.len() - 1 != size{
            return None;
        }

        let rd = search_register_by_name(parts[1].trim().to_string(), register_bank).expect("Invalid destiny register");

        let rn = if size == 3 {
            Some(search_register_by_name(parts[2].trim().to_string(), register_bank).expect("Invalid operator register"))
        } else {
            None
        };

        let operand2 = parts.last().unwrap().trim().to_string();

        let operand2: i64 = if (
            (operand2.to_lowercase().chars().nth(0).unwrap() == 'r')) && 
            ((operand2.len() == 2  && !operand2.chars().nth(1).unwrap().is_alphabetic()) || 
            (operand2.len() == 3  && !operand2.chars().nth(1).unwrap().is_alphabetic()  && !operand2.chars().nth(2).unwrap().is_alphabetic())) {

            let op_value: i64 = search_register_by_name(operand2, register_bank).unwrap_or_else(|| {panic!("Invalid register for operand")}).read();

            op_value
            
        } else {
            let op_value: i64 = operand2.parse().unwrap_or_else(|_|{0});
            op_value
        };

        Some(Self{operation: op.trim().to_string(), rd: rd.clone(), rn: rn.clone().cloned(), operand2})  //Change or understand why .clone
    }
}

fn search_register_by_name(register_name: String, register_bank: &HashSet<Register>) -> Option<&Register>{
    for r in register_bank {
        if r.name == register_name {
            return Some(r);
        }
    }

    None
}