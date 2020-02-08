from enum import Enum, unique


@unique
class InstructionType(Enum):
    MOV = 0x0001
    LD = 0x0002
    STR = 0x0003

    # arithmetic
    ADD = 0x0101
    SUB = 0x0102
    INC = 0x0103
    DEC = 0x0104
    DIV = 0x0105
    MUL = 0x0106
    MOD = 0x0107

    # binary
    AND = 0x0201
    OR = 0x0202
    XOR = 0x0203
    NOT = 0x0204

    # compare & jumps
    JMP = 0x0301
    JE = 0x0302
    JNE = 0x0303

    # conditional jumps
    JLS = 0x0401
    JLU = 0x0402
    JLES = 0x0403
    JLEU = 0x0404
    JGS = 0x0405
    JGU = 0x0406
    JGES = 0x0407
    JGEU = 0x0408

    # stack
    PUSH = 0x0501
    POP = 0x0502
    CALL = 0x0503
    RET = 0x0504

    # io
    IN = 0x0601
    OUT = 0x0602

    # syscalls
    SYSCALL = 0x0701
    SYSRET = 0x0702

    # other
    HALT = 0x0801


@unique
class ParameterType(Enum):
    REGISTER = 0x1
    IMMEDIATE_ONE_BYTE = 0x2
    IMMEDIATE_TWO_BYTE = 0x3
    IMMEDIATE_FOUR_BYTE = 0x4
    IMMEDIATE_EIGHT_BYTE = 0x5


@unique
class InstructionSize(Enum):
    ONE_BYTE = 0x1
    TWO_BYTE = 0x2
    FOUR_BYTE = 0x3
    EIGHT_BYTE = 0x4

    def __str__(self):
        if self == self.ONE_BYTE:
            return '1'
        elif self == self.TWO_BYTE:
            return '2'
        elif self == self.FOUR_BYTE:
            return '4'
        elif self == self.EIGHT_BYTE:
            return '8'

    @classmethod
    def from_string(cls, string):
        if string == '1':
            return cls.ONE_BYTE
        elif string == '2':
            return cls.TWO_BYTE
        elif string == '4':
            return cls.FOUR_BYTE
        elif string == '8':
            return cls.EIGHT_BYTE
        else:
            raise ValueError(f'Invalid value for InstructionSize: {string}')


@unique
class Register(Enum):
    I0 = 0x01
    I1 = 0x02
    I2 = 0x03
    I3 = 0x04
    I4 = 0x05
    I5 = 0x06
    I6 = 0x07
    I7 = 0x08
