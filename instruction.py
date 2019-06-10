from typing import Union, List

from instruction_set import InstructionType, ParameterType, Register


class InstructionParam:
    def __init__(self, parameter_type: ParameterType, value: Union[int, Register]):
        self.parameter_type = parameter_type
        self.value = value
        if self.parameter_type == ParameterType.REGISTER and not isinstance(value, Register):
            raise ValueError('Invalid value for register instruction param')

        if self.parameter_type != ParameterType.REGISTER:
            if not isinstance(value, int):
                raise ValueError('Invalid value for instruction param')
            if value < 0:
                raise ValueError(
                    'Instruction param value cannot be smaller than 0')

            if self.parameter_type == ParameterType.ADDRESS and value > 2**64:
                raise ValueError('invalid value for parameter type address')
            if self.parameter_type == ParameterType.IMMEDIATE_ONE_BYTE and value > 2**8:
                raise ValueError('invalid value for parameter type immediate one byte')
            if self.parameter_type == ParameterType.IMMEDIATE_TWO_BYTE and value > 2**16:
                raise ValueError('invalid value for parameter type immediate two byte')
            if self.parameter_type == ParameterType.IMMEDIATE_FOUR_BYTE and value > 2**32:
                raise ValueError('invalid value for parameter type immediate four byte')
            if self.parameter_type == ParameterType.IMMEDIATE_EIGHT_BYTE and value > 2**64:
                raise ValueError('invalid value for parameter type immediate eight byte')

    def __str__(self):
        if self.parameter_type == ParameterType.REGISTER:
            return f'%i{self.value.value - 1}'
        elif self.parameter_type == ParameterType.ADDRESS:
            return f'${self.value}'
        else:
            return str(self.value)


class Instruction:
    def __init__(self, instruction_type: InstructionType, params: List[InstructionParam]):
        self.instruction_type = instruction_type
        self.params = params

    def __str__(self):
        if self.params:
            return self.instruction_type.name.lower() + ' ' + ', '.join([str(param) for param in self.params])
        else:
            return self.instruction_type.name.lower()
