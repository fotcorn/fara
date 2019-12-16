use num_traits::FromPrimitive;

use crate::instruction::{Instruction, InstructionParam};
use crate::instruction_set::{
    InstructionSignedness, InstructionSize, InstructionType, ParameterType, Register,
};
use crate::machine_state::MachineState;

use std::convert::TryInto;

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
    let instr = &ms.memory[ms.pc as usize..ms.pc as usize + 4];
    let instr = u32::from_be_bytes(instr.try_into().unwrap());

    let instr_type = match InstructionType::from_u32((instr & TYPE) >> TYPE_OFFSET) {
        Some(t) => t,
        None => panic!("Invalid instruction type"),
    };

    let instr_size = match InstructionSize::from_u32((instr & SIZE) >> SIZE_OFFSET) {
        Some(t) => t,
        None => panic!("Invalid instruction type"),
    };

    let instr_signedness =
        match InstructionSignedness::from_u32((instr & SIGNEDNESS) >> SIGNEDNESS_OFFSET) {
            Some(t) => t,
            None => panic!("Invalid instruction type"),
        };

    let param_types = vec![
        (instr & PARAM1_TYPE) >> PARAM1_TYPE_OFFSET,
        (instr & PARAM2_TYPE) >> PARAM2_TYPE_OFFSET,
        (instr & PARAM3_TYPE) >> PARAM3_TYPE_OFFSET,
        (instr & PARAM4_TYPE) >> PARAM4_TYPE_OFFSET,
    ];

    let mut offset = 4;

    let mut instr = Instruction {
        instruction: instr_type,
        size: instr_size,
        signedness: instr_signedness,
        params: Vec::new(),
    };

    for param_type in param_types {
        let param_type = match ParameterType::from_u32(param_type) {
            Some(t) => t,
            None => panic!("Invalid instruction type"),
        };

        let address = (ms.pc + offset) as usize;

        match param_type {
            ParameterType::NoParameter => {
                return (instr, offset);
            }
            ParameterType::Register => {
                let register = ms.memory[address];
                let register = match Register::from_u8(register) {
                    Some(t) => t,
                    None => panic!("Invalid register"),
                };
                instr.params.push(InstructionParam::Register(register));
                offset += 1;
            }
            ParameterType::ImmediateOneByte => {
                let value = ms.memory[address];
                instr.params.push(InstructionParam::Immediate(value as i64));
                offset += 1;
            }
            ParameterType::ImmediateTwoByte => {
                let value = &ms.memory[address..address + 2];
                let value = value.try_into().unwrap();
                let value = u16::from_be_bytes(value);
                instr.params.push(InstructionParam::Immediate(value as i64));
                offset += 2;
            }
            ParameterType::ImmediateFourByte => {
                let value = &ms.memory[address..address + 4];
                let value = value.try_into().unwrap();
                let value = u32::from_be_bytes(value);
                instr.params.push(InstructionParam::Immediate(value as i64));
                offset += 4;
            }
            ParameterType::ImmediateEightByte => {
                let value = &ms.memory[address..address + 8];
                let value = value.try_into().unwrap();
                let value = u64::from_be_bytes(value);
                instr.params.push(InstructionParam::Immediate(value as i64));
                offset += 8;
            }
        }
    }
    (instr, offset)
}
