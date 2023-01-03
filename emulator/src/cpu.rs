use core::panic;

use crate::cpu_utils;
use crate::instruction::Instruction;
use crate::instruction_set::{InstructionSize, InstructionType};
use crate::machine_state::MachineState;

#[derive(Debug)]
pub enum ExecutionError {
    InvalidNumberOfArguments,
    InvalidInstructionSize,
}

pub fn execute(
    machine_state: &mut MachineState,
    instruction: &Instruction,
) -> Result<(), ExecutionError> {
    match instruction.instruction {
        InstructionType::MOV => {
            if instruction.params.len() != 2 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            machine_state.set_value(value, &instruction.params[1], &instruction.size);
        }
        InstructionType::LD => {
            if instruction.params.len() != 2 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let address =
                machine_state.get_value(&instruction.params[0], &InstructionSize::EightByte);
            let value = match instruction.size {
                InstructionSize::Zero => return Err(ExecutionError::InvalidInstructionSize),
                InstructionSize::OneByte => machine_state.read_memory1(address) as i64,
                InstructionSize::TwoByte => machine_state.read_memory2(address) as i64,
                InstructionSize::FourByte => machine_state.read_memory4(address) as i64,
                InstructionSize::EightByte => machine_state.read_memory8(address) as i64,
            };
            machine_state.set_value(value, &instruction.params[1], &instruction.size);
        }
        InstructionType::STR => {
            if instruction.params.len() != 2 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            let address =
                machine_state.get_value(&instruction.params[1], &InstructionSize::EightByte);
            match instruction.size {
                InstructionSize::Zero => return Err(ExecutionError::InvalidInstructionSize),
                InstructionSize::OneByte => machine_state.write_memory1(address, value as i8),
                InstructionSize::TwoByte => machine_state.write_memory2(address, value as i16),
                InstructionSize::FourByte => machine_state.write_memory4(address, value as i32),
                InstructionSize::EightByte => machine_state.write_memory8(address, value),
            };
        }

        // arithmetic
        InstructionType::ADD => {
            let (value1, value2) =
                cpu_utils::get_two_params_value(&instruction, &machine_state, "ADD");
            let result = value1 + value2;
            machine_state.set_value(result, &instruction.params[1], &instruction.size);
        }
        InstructionType::SUB => {
            let (value1, value2) =
                cpu_utils::get_two_params_value(&instruction, &machine_state, "SUB");
            let result = value1 - value2;
            machine_state.set_value(result, &instruction.params[1], &instruction.size);
        }
        InstructionType::DIV => {
            let (value1, value2) =
                cpu_utils::get_two_params_value(&instruction, &machine_state, "DIV");
            let result = value1 / value2;
            machine_state.set_value(result, &instruction.params[1], &instruction.size);
        }
        InstructionType::MUL => {
            let (value1, value2) =
                cpu_utils::get_two_params_value(&instruction, &machine_state, "MUL");
            let result = value1 * value2;
            machine_state.set_value(result, &instruction.params[1], &instruction.size);
        }
        InstructionType::MOD => {
            let (value1, value2) =
                cpu_utils::get_two_params_value(&instruction, &machine_state, "MOD");
            let result = value1 % value2;
            machine_state.set_value(result, &instruction.params[1], &instruction.size);
        }

        // bitwise
        InstructionType::AND => {
            let (value1, value2) =
                cpu_utils::get_two_params_value(&instruction, &machine_state, "AND");
            let result = value1 & value2;
            machine_state.set_value(result, &instruction.params[1], &instruction.size);
        }
        InstructionType::OR => {
            let (value1, value2) =
                cpu_utils::get_two_params_value(&instruction, &machine_state, "OR");
            let result = value1 | value2;
            machine_state.set_value(result, &instruction.params[1], &instruction.size);
        }
        InstructionType::XOR => {
            let (value1, value2) =
                cpu_utils::get_two_params_value(&instruction, &machine_state, "XOR");
            let result = value1 ^ value2;
            machine_state.set_value(result, &instruction.params[1], &instruction.size);
        }
        InstructionType::NOT => {
            if instruction.params.len() != 1 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            let result = !value;
            machine_state.set_value(result, &instruction.params[0], &instruction.size);
        }

        // jumps
        InstructionType::JMP => {
            if instruction.params.len() != 1 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            cpu_utils::conditional_jump(true, machine_state, &instruction.params[0]);
        }
        InstructionType::JE => {
            if instruction.params.len() != 3 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            cpu_utils::conditional_jump(value1 == value2, machine_state, &instruction.params[2]);
        }
        InstructionType::JNE => {
            if instruction.params.len() != 3 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            cpu_utils::conditional_jump(value1 != value2, machine_state, &instruction.params[2]);
        }
        InstructionType::JLS => {
            if instruction.params.len() != 3 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = value1 < value2;
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }
        InstructionType::JLU => {
            if instruction.params.len() != 3 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = (value1 as u64) < (value2 as u64);
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }
        InstructionType::JLES => {
            if instruction.params.len() != 3 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = value1 <= value2;
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }
        InstructionType::JLEU => {
            if instruction.params.len() != 3 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = (value1 as u64) <= (value2 as u64);
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }
        InstructionType::JGS => {
            if instruction.params.len() != 3 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = value1 > value2;
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }
        InstructionType::JGU => {
            if instruction.params.len() != 3 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = (value1 as u64) > (value2 as u64);
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }
        InstructionType::JGES => {
            if instruction.params.len() != 3 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = value1 >= value2;
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }
        InstructionType::JGEU => {
            if instruction.params.len() != 3 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = (value1 as u64) >= (value2 as u64);
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }

        // stack
        InstructionType::PUSH => {
            if instruction.params.len() != 1 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            match instruction.size {
                InstructionSize::Zero => return Err(ExecutionError::InvalidInstructionSize),
                InstructionSize::OneByte => {
                    machine_state.sp -= 1;
                    machine_state.write_memory1(machine_state.sp, value as i8);
                }
                InstructionSize::TwoByte => {
                    machine_state.sp -= 2;
                    machine_state.write_memory2(machine_state.sp, value as i16);
                }
                InstructionSize::FourByte => {
                    machine_state.sp -= 4;
                    machine_state.write_memory4(machine_state.sp, value as i32);
                }
                InstructionSize::EightByte => {
                    machine_state.sp -= 8;
                    machine_state.write_memory8(machine_state.sp, value);
                }
            };
        }
        InstructionType::POP => {
            if instruction.params.len() != 1 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            let value = match instruction.size {
                InstructionSize::Zero => return Err(ExecutionError::InvalidInstructionSize),
                InstructionSize::OneByte => {
                    machine_state.sp += 1;
                    machine_state.read_memory1(machine_state.sp - 1) as i64
                }
                InstructionSize::TwoByte => {
                    machine_state.sp += 2;
                    machine_state.read_memory2(machine_state.sp - 2) as i64
                }
                InstructionSize::FourByte => {
                    machine_state.sp += 4;
                    machine_state.read_memory4(machine_state.sp - 4) as i64
                }
                InstructionSize::EightByte => {
                    machine_state.sp += 8;
                    machine_state.read_memory8(machine_state.sp - 8) as i64
                }
            };
            machine_state.set_value(value, &instruction.params[0], &instruction.size);
        }
        InstructionType::CALL => {
            if instruction.params.len() != 1 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            if instruction.size != InstructionSize::EightByte {
                return Err(ExecutionError::InvalidInstructionSize);
            }
            machine_state.sp -= 8;
            machine_state.write_memory8(machine_state.sp, machine_state.pc);
            cpu_utils::conditional_jump(true, machine_state, &instruction.params[0]);
        }
        InstructionType::RET => {
            if instruction.params.len() != 0 {
                return Err(ExecutionError::InvalidNumberOfArguments);
            }
            if instruction.size != InstructionSize::Zero {
                return Err(ExecutionError::InvalidInstructionSize);
            }
            let return_address = machine_state.read_memory8(machine_state.sp);
            machine_state.sp += 8;
            machine_state.pc = return_address;
        }

        // io
        InstructionType::IN => panic!("Not implemented instruction IN"),
        InstructionType::OUT => {
            let port = machine_state.get_value(&instruction.params[0], &InstructionSize::OneByte);
            let value = machine_state.get_value(&instruction.params[1], &InstructionSize::OneByte);
            if port == 1 {
                print!("{}", value as u8 as char);
            }
        }

        // syscalls
        InstructionType::SYSCALL => panic!("Not implemented instruction SYSCALL"),
        InstructionType::SYSRET => panic!("Not implemented instruction SYSRET"),

        // other
        InstructionType::HALT => {
            machine_state.halt = true;
        }
    }
    Ok(())
}
