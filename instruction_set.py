from enum import Enum, unique


@unique
class InstructionType(Enum):
    LD = 0x0001
    CP = 0x0002

    # arithmetic
    ADD = 0x0101
    SUB = 0x0102
    INC = 0x0103
    DEC = 0x0104
    DIV = 0x0105
    MUL = 0x0106
    MOD = 0x0107

    ADDU = 0x0201
    SUBU = 0x0202
    INCU = 0x0203
    DECU = 0x0204
    DIVU = 0x0205
    MULU = 0x0206
    MODU = 0x0207

    # binary
    AND = 0x0301
    OR = 0x0302
    XOR = 0x0303
    NOT = 0x034

    # compare & jumps
    JMP = 0x0401
    JE = 0x0402
    JNE = 0x0403

    # signed
    JL = 0x0501
    JLE = 0x0502
    JG = 0x0503
    JGE = 0x0504

    # unsigned
    JLU = 0x0601
    JLEU = 0x0602
    JGU = 0x0603
    JGEU = 0x0604

    # stack
    PUSH = 0x0701
    POP = 0x0702
    CALL = 0x0703
    RET = 0x0704

    # io
    IN = 0x0801
    OUT = 0x0802

    # syscalls
    SYSCALL = 0x0901
    SYSRET = 0x0902


@unique
class ParameterType(Enum):
    REGISTER = 0x1
    ADDRESS = 0x2
    IMMEDIATE_ONE_BYTE = 0x3
    IMMEDIATE_TWO_BYTE = 0x4
    IMMEDIATE_FOUR_BYTE = 0x5
    IMMEDIATE_EIGHT_BYTE = 0x6


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
