import sys
import bitstruct

from emulator.instructions.dispatch import dispatch
from emulator.machine_state import MachineState
from isa.instruction import Instruction, InstructionParam
from isa.instruction_set import InstructionType, ParameterType, Register, InstructionSize, InstructionSignedness


def run(state: MachineState):
    while state.running:
        instruction = state.memory[state.pc:state.pc+4]
        instruction, size, signedness, *param_types = bitstruct.unpack('u12u3u1' + 'u4u4u4u4', instruction)

        pc_offset = 4
        params = []
        instruction_type = InstructionType(instruction)
        for param_type in param_types:
            if param_type == 0:
                break
            param_type = ParameterType(param_type)
            if param_type == ParameterType.IMMEDIATE_EIGHT_BYTE:
                data = bitstruct.unpack('u64', state.memory[state.pc+pc_offset:state.pc+pc_offset+8])[0]
                pc_offset += 8
            elif param_type == ParameterType.IMMEDIATE_FOUR_BYTE:
                data = bitstruct.unpack('u32', state.memory[state.pc+pc_offset:state.pc+pc_offset+4])[0]
                pc_offset += 4
            elif param_type == ParameterType.IMMEDIATE_TWO_BYTE:
                data = bitstruct.unpack('u16', state.memory[state.pc+pc_offset:state.pc+pc_offset+2])[0]
                pc_offset += 2
            elif param_type == ParameterType.IMMEDIATE_ONE_BYTE:
                data = bitstruct.unpack('u8', state.memory[state.pc+pc_offset:state.pc+pc_offset+1])[0]
                pc_offset += 1
            elif param_type == ParameterType.REGISTER:
                data = bitstruct.unpack('u8', state.memory[state.pc+pc_offset:state.pc+pc_offset+1])[0]
                pc_offset += 1
                data = Register(data)
            else:
                raise ValueError(f'invalid parameter type: {param_type}')

            params.append(InstructionParam(
                param_type,
                data
            ))

        instruction_object = Instruction(instruction_type, InstructionSize(size), InstructionSignedness(signedness),
                                         params)

        # program counter is already on the next instruction when executing the instruction
        state.pc += pc_offset

        dispatch(instruction_object, state)
