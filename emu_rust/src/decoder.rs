use crate::instruction::Instruction;
use crate::instruction_set::{InstructionSignedness, InstructionSize, InstructionType};
use crate::machine_state::MachineState;

pub fn decode(ms: &MachineState) -> (Instruction, i64) {
    (
        Instruction {
            instruction: InstructionType::ADD,
            size: InstructionSize::EightByte,
            signedness: InstructionSignedness::Unsigned,
        },
        5,
    )
}
