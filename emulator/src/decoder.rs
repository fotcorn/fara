use num_traits::FromPrimitive;

use crate::instruction::{Instruction, InstructionParam};
use crate::instruction_set::{InstructionSize, InstructionType, ParameterType, Register};
use crate::machine_state::MachineState;

use std::convert::TryInto;

const TYPE_OFFSET: u32 = 16;
const SIZE_OFFSET: u32 = 12;
const PARAM1_TYPE_OFFSET: u32 = 8;
const PARAM2_TYPE_OFFSET: u32 = 4;
const PARAM3_TYPE_OFFSET: u32 = 0;

const TYPE: u32 = 0xFFFF << TYPE_OFFSET;
const SIZE: u32 = 0b1111 << SIZE_OFFSET;
const PARAM1_TYPE: u32 = 0b1111 << PARAM1_TYPE_OFFSET;
const PARAM2_TYPE: u32 = 0b1111 << PARAM2_TYPE_OFFSET;
const PARAM3_TYPE: u32 = 0b1111 << PARAM3_TYPE_OFFSET;

pub fn decode(ms: &MachineState, print_instr: bool) -> (Instruction, i64) {
    let instr = &ms.memory[ms.pc as usize..ms.pc as usize + 4];
    let instr = u32::from_be_bytes(instr.try_into().unwrap());

    let instr_type = (instr & TYPE) >> TYPE_OFFSET;
    let instr_type = match InstructionType::from_u32(instr_type) {
        Some(t) => t,
        None => panic!("Invalid instruction type: {:x}", instr_type),
    };

    let instr_size = (instr & SIZE) >> SIZE_OFFSET;
    let instr_size = match InstructionSize::from_u32(instr_size) {
        Some(t) => t,
        None => panic!("Invalid instruction size {:?}", instr_size),
    };

    let param_types = vec![
        (instr & PARAM1_TYPE) >> PARAM1_TYPE_OFFSET,
        (instr & PARAM2_TYPE) >> PARAM2_TYPE_OFFSET,
        (instr & PARAM3_TYPE) >> PARAM3_TYPE_OFFSET,
    ];

    let mut offset = 4;

    let mut instr = Instruction {
        instruction: instr_type,
        size: instr_size,
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
                if print_instr {
                    println!("{:?}", instr);
                }
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
                let value = i16::from_be_bytes(value);
                instr.params.push(InstructionParam::Immediate(value as i64));
                offset += 2;
            }
            ParameterType::ImmediateFourByte => {
                let value = &ms.memory[address..address + 4];
                let value = value.try_into().unwrap();
                let value = i32::from_be_bytes(value);
                instr.params.push(InstructionParam::Immediate(value as i64));
                offset += 4;
            }
            ParameterType::ImmediateEightByte => {
                let value = &ms.memory[address..address + 8];
                let value = value.try_into().unwrap();
                let value = i64::from_be_bytes(value);
                instr.params.push(InstructionParam::Immediate(value as i64));
                offset += 8;
            }
        }
    }
    if print_instr {
        println!("{:?}", instr);
    }

    (instr, offset)
}
