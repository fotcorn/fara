use crate::cpu_utils;
use crate::instruction::Instruction;
use crate::instruction_set::{InstructionSignedness, InstructionSize, InstructionType};
use crate::machine_state::MachineState;

pub fn execute(machine_state: &mut MachineState, instruction: &Instruction) {
    match instruction.instruction {
        InstructionType::MOV => {
            assert!(
                instruction.params.len() == 2,
                "MOV instruction requires two arguments"
            );
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            machine_state.set_value(value, &instruction.params[1], &instruction.size);
        }
        InstructionType::LD => {
            assert!(
                instruction.params.len() == 2,
                "LD instruction requires two arguments"
            );
            let address =
                machine_state.get_value(&instruction.params[0], &InstructionSize::EightByte);
            let value = match instruction.size {
                InstructionSize::OneByte => machine_state.read_memory1(address) as i64,
                InstructionSize::TwoByte => machine_state.read_memory2(address) as i64,
                InstructionSize::FourByte => machine_state.read_memory4(address) as i64,
                InstructionSize::EightByte => machine_state.read_memory8(address) as i64,
            };
            machine_state.set_value(value, &instruction.params[1], &instruction.size);
        }
        InstructionType::STR => {
            assert!(
                instruction.params.len() == 2,
                "STR instruction requires two arguments"
            );
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            let address =
                machine_state.get_value(&instruction.params[1], &InstructionSize::EightByte);
            match instruction.size {
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
        InstructionType::INC => {
            assert!(
                instruction.params.len() == 1,
                "INC instruction requires one argument"
            );
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            let result = value + 1;
            machine_state.set_value(result, &instruction.params[0], &instruction.size);
        }
        InstructionType::DEC => {
            assert!(
                instruction.params.len() == 1,
                "DEC instruction requires one argument"
            );
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            let result = value - 1;
            machine_state.set_value(result, &instruction.params[0], &instruction.size);
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
            assert!(
                instruction.params.len() == 1,
                "NOT instruction requires one argument"
            );
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            let result = !value;
            machine_state.set_value(result, &instruction.params[0], &instruction.size);
        }

        // jumps
        InstructionType::JMP => {
            assert!(
                instruction.params.len() == 1,
                "JMP instruction requires one argument"
            );
            cpu_utils::conditional_jump(true, machine_state, &instruction.params[0]);
        }
        InstructionType::JE => {
            assert!(
                instruction.params.len() == 3,
                "JE instruction requires three arguments"
            );
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            cpu_utils::conditional_jump(value1 == value2, machine_state, &instruction.params[2]);
        }
        InstructionType::JNE => {
            assert!(
                instruction.params.len() == 3,
                "JNE instruction requires three arguments"
            );
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            cpu_utils::conditional_jump(value1 != value2, machine_state, &instruction.params[2]);
        }
        InstructionType::JL => {
            assert!(
                instruction.params.len() == 3,
                "JL instruction requires three arguments"
            );
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = match instruction.signedness {
                InstructionSignedness::Signed => value1 < value2,
                InstructionSignedness::Unsigned => (value1 as u64) < (value2 as u64),
            };
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }
        InstructionType::JLE => {
            assert!(
                instruction.params.len() == 3,
                "JLE instruction requires three arguments"
            );
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = match instruction.signedness {
                InstructionSignedness::Signed => value1 <= value2,
                InstructionSignedness::Unsigned => (value1 as u64) <= (value2 as u64),
            };
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }
        InstructionType::JG => {
            assert!(
                instruction.params.len() == 3,
                "JG instruction requires three arguments"
            );
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = match instruction.signedness {
                InstructionSignedness::Signed => value1 > value2,
                InstructionSignedness::Unsigned => (value1 as u64) > (value2 as u64),
            };
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }
        InstructionType::JGE => {
            assert!(
                instruction.params.len() == 3,
                "JGE instruction requires three arguments"
            );
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            let result = match instruction.signedness {
                InstructionSignedness::Signed => value1 >= value2,
                InstructionSignedness::Unsigned => (value1 as u64) >= (value2 as u64),
            };
            cpu_utils::conditional_jump(result, machine_state, &instruction.params[2]);
        }

        // stack
        InstructionType::PUSH => {
            assert!(
                instruction.params.len() == 1,
                "PUSH instruction requires one argument"
            );
            let value = machine_state.get_value(&instruction.params[0], &instruction.size);
            match instruction.size {
                InstructionSize::OneByte => {
                    machine_state.write_memory1(machine_state.sp, value as i8);
                    machine_state.sp += 1;
                }
                InstructionSize::TwoByte => {
                    machine_state.write_memory2(machine_state.sp, value as i16);
                    machine_state.sp += 2;
                }
                InstructionSize::FourByte => {
                    machine_state.write_memory4(machine_state.sp, value as i32);
                    machine_state.sp += 4;
                }
                InstructionSize::EightByte => {
                    machine_state.write_memory8(machine_state.sp, value);
                    machine_state.sp += 8;
                }
            };
        }
        InstructionType::POP => {
            assert!(
                instruction.params.len() == 1,
                "POP instruction requires one argument"
            );
            let value = match instruction.size {
                InstructionSize::OneByte => {
                    machine_state.sp -= 1;
                    machine_state.read_memory1(machine_state.sp) as i64
                }
                InstructionSize::TwoByte => {
                    machine_state.sp -= 2;
                    machine_state.read_memory2(machine_state.sp) as i64
                }
                InstructionSize::FourByte => {
                    machine_state.sp -= 4;
                    machine_state.read_memory4(machine_state.sp) as i64
                }
                InstructionSize::EightByte => {
                    machine_state.sp -= 8;
                    machine_state.read_memory8(machine_state.sp) as i64
                }
            };
            machine_state.set_value(value, &instruction.params[0], &instruction.size);
        }
        InstructionType::CALL => {
            assert!(
                instruction.params.len() == 1,
                "CALL instruction requires one argument"
            );
            machine_state.write_memory8(machine_state.sp, machine_state.pc);
            machine_state.sp += 8;
            cpu_utils::conditional_jump(true, machine_state, &instruction.params[0]);
        }
        InstructionType::RET => {
            assert!(
                instruction.params.len() == 0,
                "RET instruction does not take any arguemnts"
            );
            let value = match instruction.size {
                InstructionSize::OneByte => {
                    machine_state.sp -= 1;
                    machine_state.read_memory1(machine_state.sp) as i64
                }
                InstructionSize::TwoByte => {
                    machine_state.sp -= 2;
                    machine_state.read_memory2(machine_state.sp) as i64
                }
                InstructionSize::FourByte => {
                    machine_state.sp -= 4;
                    machine_state.read_memory4(machine_state.sp) as i64
                }
                InstructionSize::EightByte => {
                    machine_state.sp -= 8;
                    machine_state.read_memory8(machine_state.sp) as i64
                }
            };
            machine_state.pc = value;
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
}
