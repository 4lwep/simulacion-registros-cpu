use std::collections::HashSet;
use crate::register::Register;

pub struct Instruction{
    pub operation: String,
    pub rd_name: String,
    pub rn_name: Option<String>,
    pub operand2: i64
}

impl Instruction{
    pub fn parse(instruction: String, register_bank: &HashSet<Register>) -> Self{
        let mut operation = String::new();
        for c in instruction.chars(){
            if c == ','{
                break;
            }
            operation.push(c);
        }

        let mut rd_name = String::new();
        let mut start = false;
        for c in instruction.chars(){
            if start && c == ','{
                break;
            }
            if c == ','{
                start = true;
            }
            if start && c != ','{
                rd_name.push(c);
            }
        }

        let mut operand2 = String::new();
        let mut start = 0;
        let mut operand: i64 = 0;
        for c in instruction.chars(){
            if c == ','{
                start += 1;
            }

            if start == 2 && c != ','{
                operand2.push(c);
            }   
        }
        if operand2.len() > 0 && operand2.chars().nth(0).unwrap() == 'r'{
            
            for r in register_bank {
                if r.name == operand2{
                    operand = r.read();
                }
            }
        } else {
            operand = operand2.trim().parse().unwrap_or_else(|_| {
                0
            });
        }
        Instruction { operation, rd_name, rn_name: None, operand2: operand }
    }
}