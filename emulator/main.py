import sys
import struct

from emulator.exceptions import HaltException
from emulator.instructions.dispatch import dispatch
from emulator.machine_state import MachineState
from isa.instruction import Instruction, InstructionParam
from isa.instruction_set import InstructionType, ParameterType, Register


class Interpreter:
    def __init__(self):
        self.state = MachineState()


def main():
    if len(sys.argv) != 2:
        print(f'{sys.argv[0]} <file.bin>')
        sys.exit(1)
    with open(sys.argv[1], 'rb') as f:
        code = f.read()

    state = MachineState()
    state.memory[1000:1000+len(code)] = code
    pc = 1000

    while True:
        instruction = state.memory[pc:pc+5]
        instruction, *param_types = struct.unpack('<HBBB', instruction)

        pc_offset = 5
        params = []
        instruction = InstructionType(instruction)
        for param_type in param_types:
            if param_type == 0:
                break
            param_type = ParameterType(param_type)
            if param_type == ParameterType.ADDRESS:
                data = struct.unpack('<Q', state.memory[pc+pc_offset:pc+pc_offset+8])[0]
                pc_offset += 8
            elif param_type == ParameterType.IMMEDIATE_EIGHT_BYTE:
                data = struct.unpack('<Q', state.memory[pc+pc_offset:pc+pc_offset+8])[0]
                pc_offset += 8
            elif param_type == ParameterType.IMMEDIATE_FOUR_BYTE:
                data = struct.unpack('<I', state.memory[pc+pc_offset:pc+pc_offset+4])[0]
                pc_offset += 4
            elif param_type == ParameterType.IMMEDIATE_TWO_BYTE:
                data = struct.unpack('<H', state.memory[pc+pc_offset:pc+pc_offset+2])[0]
                pc_offset += 2
            elif param_type == ParameterType.IMMEDIATE_ONE_BYTE:
                data = struct.unpack('<B', state.memory[pc+pc_offset:pc+pc_offset+1])[0]
                pc_offset += 1
            elif param_type == ParameterType.REGISTER:
                data = struct.unpack('<B', state.memory[pc+pc_offset:pc+pc_offset+1])[0]
                pc_offset += 1
                data = Register(data)
            else:
                raise ValueError(f'invalid parameter type: {param_type}')

            params.append(InstructionParam(
                param_type,
                data
            ))

        instruction = Instruction(instruction, params)
        try:
            dispatch(instruction, state)
        except HaltException:
            return
        pc += pc_offset


if __name__ == '__main__':
    main()
