from isa.instruction import InstructionParam
from isa.instruction_set import ParameterType, Register


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

    memory: bytearray = bytearray(10000)

    stdin = []

    running = True

    def get_value(self, param: InstructionParam):
        if param.parameter_type == ParameterType.REGISTER:
            if param.value == Register.I0:
                return self.i0
            elif param.value == Register.I1:
                return self.i1
            elif param.value == Register.I2:
                return self.i2
            elif param.value == Register.I3:
                return self.i3
            elif param.value == Register.I4:
                return self.i4
            elif param.value == Register.I5:
                return self.i5
            elif param.value == Register.I6:
                return self.i6
            elif param.value == Register.I7:
                return self.i7
            else:
                raise ValueError('Unknown register')
        elif param.parameter_type == ParameterType.ADDRESS:
            raise NotImplementedError('MachineState.get_value with ADDRESS parameter type not implemented')
        elif param.parameter_type == ParameterType.IMMEDIATE_ONE_BYTE:
            return param.value
        elif param.parameter_type == ParameterType.IMMEDIATE_TWO_BYTE:
            return param.value
        elif param.parameter_type == ParameterType.IMMEDIATE_FOUR_BYTE:
            return param.value
        elif param.parameter_type == ParameterType.IMMEDIATE_EIGHT_BYTE:
            return param.value

    def set_value(self, param: InstructionParam, value):
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
        elif param.parameter_type == ParameterType.ADDRESS:
            raise NotImplementedError('MachineState.get_value with ADDRESS parameter type not implemented')
        elif param.parameter_type == ParameterType.IMMEDIATE_ONE_BYTE or ParameterType.IMMEDIATE_TWO_BYTE or \
                ParameterType.IMMEDIATE_FOUR_BYTE or ParameterType.IMMEDIATE_EIGHT_BYTE:
            raise ValueError('set_value: cannot set value on immediate parameter')
