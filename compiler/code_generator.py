from typing import List
import bitstruct

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
            assert len(instruction.params) <= 4

            param_types = []
            param_data = b''

            # an instruction can have up to 3 parameters
            # parameter types are appended after instruction type which is 2 bytes long
            for i in range(4):
                try:
                    param = instruction.params[i]
                except IndexError:
                    # unused parameter
                    param_types.append(0)
                    continue

                if isinstance(param, LabelRef):
                    label_placeholders.append((
                        len(code),
                        len(code) + 4 + len(param_data),
                        param.label
                    ))
                    param_types.append(ParameterType.IMMEDIATE_FOUR_BYTE.value)
                    param_data += bitstruct.pack('u32', 0)
                else:
                    param_types.append(param.parameter_type.value)

                    if param.parameter_type == ParameterType.ADDRESS:
                        param_data += bitstruct.pack('u64', param.value)
                    elif param.parameter_type == ParameterType.IMMEDIATE_EIGHT_BYTE:
                        param_data += bitstruct.pack('s64', param.value)
                    elif param.parameter_type == ParameterType.IMMEDIATE_FOUR_BYTE:
                        param_data += bitstruct.pack('s32', param.value)
                    elif param.parameter_type == ParameterType.IMMEDIATE_TWO_BYTE:
                        param_data += bitstruct.pack('s16', param.value)
                    elif param.parameter_type == ParameterType.IMMEDIATE_ONE_BYTE:
                        param_data += bitstruct.pack('s8', param.value)
                    elif param.parameter_type == ParameterType.REGISTER:
                        param_data += bitstruct.pack('u8', param.value.value)

            # 14 bits instruction type, 3 bits data size, 1 bit signedness
            code += bitstruct.pack('u12u3u1' + 'u4u4u4u4',
                                   instruction.instruction_type.value,
                                   instruction.size.value,
                                   instruction.signedness.value,
                                   *param_types)
            code += param_data
        elif isinstance(tree_entry, Label):
            labels[tree_entry.label] = len(code)

    for instr_position, replace_position, label in label_placeholders:
        relative = labels[label] - instr_position
        relative = bitstruct.pack('s32', relative)
        code[replace_position] = relative[0]
        code[replace_position + 1] = relative[1]
        code[replace_position + 2] = relative[2]
        code[replace_position + 3] = relative[3]
    return code
