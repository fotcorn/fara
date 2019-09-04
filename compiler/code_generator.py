from typing import List
import struct

from compiler.parser import Label, LabelRef
from isa.instruction import Instruction
from isa.instruction_set import ParameterType


def generate_code(tree: List[Instruction]):
    labels = {}
    label_placeholders = []

    code = bytearray()
    for tree_entry in tree:
        if isinstance(tree_entry, Instruction):
            instruction = tree_entry
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

                if isinstance(param, LabelRef):
                    label_placeholders.append((
                        len(code),
                        len(code) + 5 + len(param_data),
                        param.label
                    ))
                    param_types.append(ParameterType.IMMEDIATE_FOUR_BYTE.value)
                    param_data += struct.pack('<I', 0)
                else:
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
        elif isinstance(tree_entry, Label):
            labels[tree_entry.label] = len(code)

    for instr_position, replace_position, label in label_placeholders:
        relative = labels[label] - instr_position
        relative = struct.pack('<i', relative)
        code[replace_position] = relative[0]
        code[replace_position + 1] = relative[1]
        code[replace_position + 2] = relative[2]
        code[replace_position + 3] = relative[3]
    return code
