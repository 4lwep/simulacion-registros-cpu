use std::ops::{BitAnd, BitOr, BitXor};
use super::movement::*;

use crate::Register;

pub fn and(rd: &mut Register, rn: Register, operand2: i64){
    mov(rd, &rn.read().bitand(operand2));
}

pub fn eor(rd: &mut Register, rn: Register, operand2: i64){
    mov(rd, &rn.read().bitxor(operand2));
}

pub fn orr(rd: &mut Register, rn: Register, operand2: i64){
    mov(rd, &rn.read().bitor(operand2))
}