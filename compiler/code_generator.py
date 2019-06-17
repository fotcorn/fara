from typing import List
import struct

from isa.instruction import Instruction
from isa.instruction_set import ParameterType


def generate_code(tree: List[Instruction]):
    code = b''
    for instruction in tree:
        assert len(instruction.params) <= 3

        param_types = []
        param_data = b''

        # an instruction can have up to 3 parameters
        # parameter types are appended after instruction type which is 2 bytes long
        for i in range(3):
            try:
                param = instruction.params[i]
            except IndexError:
                # unused parameter
                param_types.append(0)
                continue

            param_types.append(param.parameter_type.value)

            if param.parameter_type == ParameterType.ADDRESS:
                param_data += struct.pack('<Q', param.value)
            elif param.parameter_type == ParameterType.IMMEDIATE_EIGHT_BYTE:
                param_data += struct.pack('<Q', param.value)
            elif param.parameter_type == ParameterType.IMMEDIATE_FOUR_BYTE:
                param_data += struct.pack('<I', param.value)
            elif param.parameter_type == ParameterType.IMMEDIATE_TWO_BYTE:
                param_data += struct.pack('<H', param.value)
            elif param.parameter_type == ParameterType.IMMEDIATE_ONE_BYTE:
                param_data += struct.pack('<B', param.value)
            elif param.parameter_type == ParameterType.REGISTER:
                param_data += struct.pack('<B', param.value.value)

        code += struct.pack('<HBBB', instruction.instruction_type.value, *param_types)
        code += param_data
    return code
