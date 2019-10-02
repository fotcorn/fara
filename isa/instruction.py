from typing import Union, List

from isa.instruction_set import InstructionType, ParameterType, Register, InstructionSignedness, InstructionSize


class InstructionParam:
    def __init__(self, parameter_type: ParameterType, value: Union[int, Register]):
        self.parameter_type = parameter_type
        self.value = value
        if self.parameter_type == ParameterType.REGISTER:
            if not isinstance(value, Register):
                raise ValueError('Invalid value for register instruction param')
        else:
            if not isinstance(value, int):
                raise ValueError('Invalid value for instruction param')

            if self.parameter_type == ParameterType.IMMEDIATE_ONE_BYTE and value > 2**8:
                raise ValueError('invalid value for parameter type immediate one byte')
            elif self.parameter_type == ParameterType.IMMEDIATE_TWO_BYTE and value > 2**16:
                raise ValueError('invalid value for parameter type immediate two byte')
            elif self.parameter_type == ParameterType.IMMEDIATE_FOUR_BYTE and value > 2**32:
                raise ValueError('invalid value for parameter type immediate four byte')
            elif self.parameter_type == ParameterType.IMMEDIATE_EIGHT_BYTE and value > 2**64:
                raise ValueError('invalid value for parameter type immediate eight byte')

    def __str__(self):
        if self.parameter_type == ParameterType.REGISTER:
            return f'%i{self.value.value - 1}'
        else:
            return str(self.value)


class Instruction:
    def __init__(self, instruction_type: InstructionType, size: InstructionSize, signedness: InstructionSignedness,
                 params: List[InstructionParam]):
        self.instruction_type = instruction_type
        self.params = params
        self.size = size
        self.signedness = signedness

    def __str__(self):
        instruction = '{}{}{}'.format(self.instruction_type.name.lower(), self.size, self.signedness)
        if self.params:
            return instruction + ' ' + ', '.join([str(param) for param in self.params])
        else:
            return instruction
