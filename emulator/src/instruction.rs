use crate::instruction_set::InstructionSize;
use crate::instruction_set::InstructionType;
use crate::instruction_set::Register;

#[derive(Debug)]
pub enum InstructionParam {
    Immediate(i64),
    Register(Register),
}

#[derive(Debug)]
pub struct Instruction {
    pub instruction: InstructionType,
    pub size: InstructionSize,
    pub params: Vec<InstructionParam>,
}
