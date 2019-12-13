extern crate zero;

use num_traits::FromPrimitive;

use crate::instruction::Instruction;
use crate::instruction_set::{InstructionSignedness, InstructionSize, InstructionType};
use crate::machine_state::MachineState;

const TYPE_OFFSET: u32 = 20;
const SIZE_OFFSET: u32 = 17;
const SIGNEDNESS_OFFSET: u32 = 16;
const PARAM1_TYPE_OFFSET: u32 = 12;
const PARAM2_TYPE_OFFSET: u32 = 8;
const PARAM3_TYPE_OFFSET: u32 = 4;
const PARAM4_TYPE_OFFSET: u32 = 0;

const TYPE: u32 = 0b111111111111 << TYPE_OFFSET;
const SIZE: u32 = 0b111 << SIZE_OFFSET;
const SIGNEDNESS: u32 = 0b1 << SIGNEDNESS_OFFSET;
const PARAM1_TYPE: u32 = 0b1111 << PARAM1_TYPE_OFFSET;
const PARAM2_TYPE: u32 = 0b1111 << PARAM2_TYPE_OFFSET;
const PARAM3_TYPE: u32 = 0b1111 << PARAM3_TYPE_OFFSET;
const PARAM4_TYPE: u32 = 0b1111 << PARAM4_TYPE_OFFSET;

pub fn decode(ms: &MachineState) -> (Instruction, i64) {
    let instr = &ms.memory[ms.pc as usize..=ms.pc as usize + 4];
    let instr = *zero::read::<u32>(instr);

    let instr_type = match InstructionType::from_u32(instr & TYPE >> TYPE_OFFSET) {
        Some(t) => t,
        None => panic!("Invalid instruction type"),
    };

    let instr_size = match InstructionSize::from_u32(instr & SIZE >> SIZE_OFFSET) {
        Some(t) => t,
        None => panic!("Invalid instruction type"),
    };

    let instr_signedness =
        match InstructionSignedness::from_u32(instr & SIGNEDNESS >> SIGNEDNESS_OFFSET) {
            Some(t) => t,
            None => panic!("Invalid instruction type"),
        };

    (
        Instruction {
            instruction: instr_type,
            size: instr_size,
            signedness: instr_signedness,
        },
        5,
    )
}
