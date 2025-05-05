use std::collections::HashSet;
use crate::register::Register;

pub struct Instruction{
    pub operation: String,
    pub rd_name: String,
    pub rn: Option<Register>,
    pub operand2: i64
}

impl Instruction{
    pub fn parse(instruction: String, register_bank: &HashSet<Register>) -> Self{

        let parts = instruction.split(',');
        let collection: Vec<&str> = parts.collect();
        assert!(collection.len() > 1);
        let operation = collection[0].trim().to_string();
        let rd_name = collection[1].trim().to_string();
        let mut rn: Option<Register> = None;
        
        if collection.len() == 4{
            let rn_name = collection[2].trim().to_string();
            
            for r in register_bank{
                if r.name == rn_name{
                    rn = Some(r.clone());
                }
            }
        }

        let operand2 = collection.last().unwrap().trim().to_string();

        let mut operand = 0;
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
        Instruction { operation, rd_name, rn, operand2: operand }
    }
}
