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
        InstructionType::ADD => panic!("Not implemented instruction ADD"),
        InstructionType::SUB => panic!("Not implemented instruction SUB"),
        InstructionType::INC => panic!("Not implemented instruction INC"),
        InstructionType::DEC => panic!("Not implemented instruction DEC"),
        InstructionType::DIV => panic!("Not implemented instruction DIV"),
        InstructionType::MUL => panic!("Not implemented instruction MUL"),
        InstructionType::MOD => panic!("Not implemented instruction MOD"),

        // binary
        InstructionType::AND => panic!("Not implemented instruction AND"),
        InstructionType::OR => panic!("Not implemented instruction OR"),
        InstructionType::XOR => panic!("Not implemented instruction XOR"),
        InstructionType::NOT => panic!("Not implemented instruction NOT"),

        // compare & jumps
        InstructionType::JMP => panic!("Not implemented instruction JMP"),
        InstructionType::JE => panic!("Not implemented instruction JE"),
        InstructionType::JNE => panic!("Not implemented instruction JNE"),

        // conditional jumps
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
        InstructionType::HALT => panic!("Not implemented instruction HALT"),
    }
}
