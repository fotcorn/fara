#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum InstructionType {
    MOV = 0x0001,
    LD = 0x0002,
    STR = 0x0003,

    // arithmetic
    ADD = 0x0101,
    SUB = 0x0102,
    MUL = 0x0103,
    DIV = 0x0104,
    MOD = 0x0105,

    // binary
    AND = 0x0201,
    OR = 0x0202,
    XOR = 0x0203,
    NOT = 0x0204,

    // compare & jumps
    JMP = 0x0301,
    JE = 0x0302,
    JNE = 0x0303,

    // conditional jumps
    JLS = 0x0401,
    JLU = 0x0402,
    JLES = 0x0403,
    JLEU = 0x0404,
    JGS = 0x0405,
    JGU = 0x0406,
    JGES = 0x0407,
    JGEU = 0x0408,

    // stack
    PUSH = 0x0501,
    POP = 0x0502,
    CALL = 0x0503,
    RET = 0x0504,

    // io
    IN = 0x0601,
    OUT = 0x0602,

    // syscalls
    SYSCALL = 0x0701,
    SYSRET = 0x0702,

    // other
    HALT = 0x0801,
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum ParameterType {
    NoParameter = 0x0,
    Register = 0x1,
    ImmediateOneByte = 0x2,
    ImmediateTwoByte = 0x3,
    ImmediateFourByte = 0x4,
    ImmediateEightByte = 0x5,
}

#[derive(PartialEq, FromPrimitive, ToPrimitive, Debug)]
pub enum InstructionSize {
    Zero = 0x0,
    OneByte = 0x1,
    TwoByte = 0x2,
    FourByte = 0x3,
    EightByte = 0x4,
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Register {
    PC = 0x01, // program counter
    SP = 0x02, // stack pointer
    FP = 0x03, // frame pointer

    // caller saved, function arguments
    I0 = 0x04, // return value & argument 1
    I1 = 0x05,
    I2 = 0x06,
    I3 = 0x07,
    I4 = 0x08,
    I5 = 0x09,
    I6 = 0x0A,
    I7 = 0x0B,

    // caller saved, temporaries
    I8 = 0x0C,
    I9 = 0x0D,
    I10 = 0x0E,
    I11 = 0x0F,
    I12 = 0x10,
    I13 = 0x11,
    I14 = 0x12,
    I15 = 0x13,

    // callee saved
    I16 = 0x14,
    I17 = 0x15,
    I18 = 0x16,
    I19 = 0x17,
    I20 = 0x18,
    I21 = 0x19,
    I22 = 0x1A,
    I23 = 0x1B,
}
