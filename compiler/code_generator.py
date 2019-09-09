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
            current_label_placeholders = []

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
                    current_label_placeholders.append({
                        'next_instruction_address': 0,
                        'replacement_address': len(code) + 4 + len(param_data),
                        'label': param.label,
                    })
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

            for placeholder in current_label_placeholders:
                placeholder['next_instruction_address'] = len(code)
            label_placeholders.extend(current_label_placeholders)

        elif isinstance(tree_entry, Label):
            labels[tree_entry.label] = len(code)

    for placeholder in label_placeholders:
        relative = labels[placeholder['label']] - placeholder['next_instruction_address']
        relative = bitstruct.pack('s32', relative)
        replacement_address = placeholder['replacement_address']
        code[replacement_address] = relative[0]
        code[replacement_address + 1] = relative[1]
        code[replacement_address + 2] = relative[2]
        code[replacement_address + 3] = relative[3]
    return code
