use crate::cpu_utils;
use crate::instruction::Instruction;
use crate::instruction_set::{InstructionSize, InstructionType};
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
            let jump_offset = machine_state.get_value(&instruction.params[0], &instruction.size);
            machine_state.pc += jump_offset;
        }
        InstructionType::JE => {
            assert!(
                instruction.params.len() == 3,
                "JE instruction requires three argument"
            );
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            if value1 == value2 {
                let jump_offset =
                    machine_state.get_value(&instruction.params[2], &instruction.size);
                machine_state.pc += jump_offset;
            }
        }
        InstructionType::JNE => {
            assert!(
                instruction.params.len() == 3,
                "JNE instruction requires three argument"
            );
            let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
            let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
            if value1 != value2 {
                let jump_offset =
                    machine_state.get_value(&instruction.params[2], &instruction.size);
                machine_state.pc += jump_offset;
            }
        }
        InstructionType::JL => panic!("Not implemented instruction JL"),
        InstructionType::JLE => panic!("Not implemented instruction JLE"),
        InstructionType::JG => panic!("Not implemented instruction JG"),
        InstructionType::JGE => panic!("Not implemented instruction JGE"),

        // stack
        InstructionType::PUSH => panic!("Not implemented instruction PUSH"),
        InstructionType::POP => panic!("Not implemented instruction POP"),
        InstructionType::CALL => panic!("Not implemented instruction CALL"),
        InstructionType::RET => panic!("Not implemented instruction RET"),

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
