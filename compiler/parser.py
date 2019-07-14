import sys
import codecs
from lark import Lark, Transformer, v_args
from isa.instruction_set import Register, InstructionType
from isa.instruction import InstructionParam, ParameterType, Instruction
import ctypes

GRAMMAR = '''start: (_statement? COMMENT? NEWLINE)* _statement? _NEWLINE?

            number: INT
            address: "$"INT
            REGISTER_NUMBER: "0".."7"
            register: "%i"REGISTER_NUMBER
            label_ref: CNAME
            char: "'"/[^']/"'"
            escape_char: "'\\\\"/[^']/"'"
            ?param: (number|address|register|label_ref|char|escape_char)

            instruction: WORD (param (","param)*)?
            label: CNAME":"

            _statement: instruction|label

            COMMENT: "//"/[^\\n]*/
            _NEWLINE: COMMENT? NEWLINE

            %import common.WORD
            %import common.CNAME
            %import common.INT
            %import common.WS_INLINE
            %import common.NEWLINE

            %ignore WS_INLINE
            %ignore COMMENT
         '''


class Label:
    def __init__(self, label):
        self.label = label

    def __str__(self):
        return f'{self.label}:'


class LabelRef:
    def __init__(self, label):
        self.label = label

    def __str__(self):
        return f'{self.label}'


class ASMTransformer(Transformer):
    def __init__(self):
        self.instruction_counter = 0
        self.labels = {}

    @v_args(inline=True)
    def address(self, address):
        return InstructionParam(ParameterType.ADDRESS, int(address))

    @v_args(inline=True)
    def number(self, number):
        number = int(number)
        if -2**7 <= number <= 2**7-1:
            return InstructionParam(ParameterType.IMMEDIATE_ONE_BYTE, int(number))
        if -2**15 <= number <= 2**15-1:
            return InstructionParam(ParameterType.IMMEDIATE_TWO_BYTE, int(number))
        if -2**31 <= number <= 2**31-1:
            return InstructionParam(ParameterType.IMMEDIATE_FOUR_BYTE, int(number))
        if -2**63 <= number <= 2**63-1:
            return InstructionParam(ParameterType.IMMEDIATE_EIGHT_BYTE, int(number))

    @v_args(inline=True)
    def char(self, char):
        return InstructionParam(ParameterType.IMMEDIATE_ONE_BYTE, ord(char))

    @v_args(inline=True)
    def escape_char(self, char):
        char = codecs.escape_decode('\\' + char)[0]
        return InstructionParam(ParameterType.IMMEDIATE_ONE_BYTE, ord(char))

    @v_args(inline=True)
    def register(self, register):
        return InstructionParam(ParameterType.REGISTER, Register(int(register) + 1))

    def instruction(self, args):
        self.instruction_counter += 1
        instruction, *params = args
        instruction = instruction.value.upper()
        instruction_type = InstructionType[instruction]
        return Instruction(instruction_type, params)

    @v_args(inline=True)
    def label_ref(self, label):
        return LabelRef(str(label))

    @v_args(inline=True)
    def label(self, label):
        return Label(str(label))

    def start(self, args):
        return args


def parse(code):
    lark = Lark(GRAMMAR)
    tree = lark.parse(code)
    transformer = ASMTransformer()
    return transformer.transform(tree)
