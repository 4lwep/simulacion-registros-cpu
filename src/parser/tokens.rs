use crate::register::Register;

enum Operation {
    MOV, 
    MVN,
    ADD,
    SUB,
    MUL,
    RSB,
    AND,
    EOR,
    ORR
}

enum RegisterName {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    CPSR
}

pub struct Op{
    o_type: Operation,
    arg_count: i8
}

pub struct Reg{
    r_type: RegisterName,
    reg: Register
}

pub struct Val{
    value: i64
}

pub struct Instruction{
    pub operation: Op,
    pub rd_name: Reg,
    pub rn: Option<Reg>,
    pub operand2: Val
}

pub struct InstructionStack{
    tokens: Vec<Instruction>
}

impl InstructionStack{
    pub fn new_empty() -> Self {
        Self { tokens: Vec::new() }
    }
}

/*
0. TOKEN ADD LEN 2
1. TOKEN
2. TOKEN
0. token add len 2 || 3
*/