import sys
import codecs
from lark import Lark, Transformer, v_args, Token
from isa.instruction_set import Register, InstructionType, InstructionSize
from isa.instruction import InstructionParam, ParameterType, Instruction
import ctypes

GRAMMAR = '''start: (_statement? COMMENT? NEWLINE)* _statement? _NEWLINE?

            number: "$"?INT
            REGISTER_NUMBER: "0".."7"
            register: "%i"REGISTER_NUMBER
            label_ref: CNAME
            char: "'"/[^']/"'"
            escape_char: "'\\\\"/[^']/"'"
            ?param: (number|register|label_ref|char|escape_char)

            INSTRUCTION_SIZE: ("1"|"2"|"4"|"8")
            instruction: WORD INSTRUCTION_SIZE?
            expression: instruction (_whitespace param (","param)*)?
            label: CNAME":"

            _statement: expression|label

            COMMENT: "//"/[^\\n]*/
            _NEWLINE: COMMENT? NEWLINE
            _whitespace: (" "|/\t/)+

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
        assert len(args) <= 2

        instruction = args[0].value.upper()
        try:
            instruction_type = InstructionType[instruction]
        except KeyError:
            raise ValueError(f'Unknown instruction: {instruction.lower()}')

        if len(args) == 1:
            size = InstructionSize.EIGHT_BYTE
        else:
            size = InstructionSize.from_string(args[1].value)
        return instruction_type, size

    def expression(self, args):
        self.instruction_counter += 1
        (instruction_type, size), *params = args
        return Instruction(instruction_type, size, params)

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
