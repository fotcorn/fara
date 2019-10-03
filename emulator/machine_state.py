from typing import List

import bitstruct

from isa.instruction import InstructionParam
from isa.instruction_set import ParameterType, Register, InstructionSize


class MachineState:
    i0: int = 0
    i1: int = 0
    i2: int = 0
    i3: int = 0
    i4: int = 0
    i5: int = 0
    i6: int = 0
    i7: int = 0

    pc: int = 0
    sp: int = 0

    memory: bytearray = bytearray(100000)

    stdin: List[int] = []

    running = True

    def get_value(self, param: InstructionParam, size: InstructionSize):
        if param.parameter_type == ParameterType.REGISTER:
            if param.value == Register.I0:
                value = self.i0
            elif param.value == Register.I1:
                value = self.i1
            elif param.value == Register.I2:
                value = self.i2
            elif param.value == Register.I3:
                value = self.i3
            elif param.value == Register.I4:
                value = self.i4
            elif param.value == Register.I5:
                value = self.i5
            elif param.value == Register.I6:
                value = self.i6
            elif param.value == Register.I7:
                value = self.i7
            else:
                raise ValueError('Unknown register')
            if size == InstructionSize.ONE_BYTE:
                return value & 0xFF
            elif size == InstructionSize.TWO_BYTE:
                return value & 0xFFFF
            elif size == InstructionSize.FOUR_BYTE:
                return value & 0xFFFFFFFF
            elif size == InstructionSize.EIGHT_BYTE:
                return value
            else:
                raise ValueError('Unknown instruction size')
        elif param.parameter_type == ParameterType.IMMEDIATE_ONE_BYTE:
            return param.value
        elif param.parameter_type == ParameterType.IMMEDIATE_TWO_BYTE:
            return param.value
        elif param.parameter_type == ParameterType.IMMEDIATE_FOUR_BYTE:
            return param.value
        elif param.parameter_type == ParameterType.IMMEDIATE_EIGHT_BYTE:
            return param.value

    def set_value(self, param: InstructionParam, value: int, size: InstructionSize):
        if size == InstructionSize.ONE_BYTE:
            value = value & 0xFF
        elif size == InstructionSize.TWO_BYTE:
            value = value & 0xFFFF
        elif size == InstructionSize.FOUR_BYTE:
            value = value & 0xFFFFFFFF
        elif size == InstructionSize.EIGHT_BYTE:
            value = value & 0xFFFFFFFFFFFFFFFF

        if param.parameter_type == ParameterType.REGISTER:
            if param.value == Register.I0:
                self.i0 = value
            elif param.value == Register.I1:
                self.i1 = value
            elif param.value == Register.I2:
                self.i2 = value
            elif param.value == Register.I3:
                self.i3 = value
            elif param.value == Register.I4:
                self.i4 = value
            elif param.value == Register.I5:
                self.i5 = value
            elif param.value == Register.I6:
                self.i6 = value
            elif param.value == Register.I7:
                self.i7 = value
            else:
                raise ValueError('Unknown register')
        elif param.parameter_type == ParameterType.IMMEDIATE_ONE_BYTE or ParameterType.IMMEDIATE_TWO_BYTE or \
                ParameterType.IMMEDIATE_FOUR_BYTE or ParameterType.IMMEDIATE_EIGHT_BYTE:
            raise ValueError('set_value: cannot set value on immediate parameter')
