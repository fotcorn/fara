use crate::instruction::{Instruction, InstructionParam};
use crate::instruction_set::InstructionSize;
use crate::machine_state::MachineState;

pub fn get_two_params_value(
    instruction: &Instruction,
    machine_state: &MachineState,
    instruction_name: &str,
) -> (i64, i64) {
    assert!(
        instruction.params.len() == 2,
        format!("{} instruction requires two arguments", instruction_name)
    );
    let value1 = machine_state.get_value(&instruction.params[0], &instruction.size);
    let value2 = machine_state.get_value(&instruction.params[1], &instruction.size);
    (value1, value2)
}

pub fn conditional_jump(result: bool, machine_state: &mut MachineState, param: &InstructionParam) {
    if result {
        let jump_offset = machine_state.get_value(param, &InstructionSize::EightByte);
        machine_state.pc += jump_offset;
    }
}
