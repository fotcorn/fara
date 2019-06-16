import sys
from lark import Lark, Transformer, v_args
from instruction_set import Register, InstructionType
from instruction import InstructionParam, ParameterType, Instruction
import ctypes

GRAMMAR = '''start: (_statement? COMMENT? NEWLINE)* _statement? _NEWLINE?

            number: INT
            address: "$"INT
            REGISTER_NUMBER: "0".."7"
            register: "%i"REGISTER_NUMBER
            label_ref: CNAME
            char: "'"/[^']/"'"
            ?param: (number|address|register|label_ref|char)

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
    def __init__(self, label, position):
        self.label = label
        self.position = position

    def __str__(self):
        return f'{self.label}: ({self.position})'


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
        return InstructionParam(ParameterType.IMMEDIATE_EIGHT_BYTE, int(number))

    @v_args(inline=True)
    def char(self, char):
        return InstructionParam(ParameterType.IMMEDIATE_EIGHT_BYTE, ord(char))

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
        return LabelRef(label)

    @v_args(inline=True)
    def label(self, label):
        if label in self.labels:
            raise ValueError('duplicated label')
        self.labels[label] = self.instruction_counter
        return None

    def start(self, args):
        instructions = []
        for arg in args:
            if isinstance(arg, Instruction) or isinstance(arg, Label):
                instructions.append(arg)
        return instructions


def main():
    if len(sys.argv) != 2:
        print(f'{sys.argv[0]} <file.asm>')
        sys.exit(1)
    with open(sys.argv[1]) as f:
        code = f.read()

    lark = Lark(GRAMMAR)
    tree = lark.parse(code)
    transformer = ASMTransformer()
    tree = transformer.transform(tree)

    # resolve labels
    for instruction_counter, instr in enumerate(tree):
        for i, param in enumerate(instr.params):
            if isinstance(param, LabelRef):
                try:
                    position = transformer.labels[param.label]
                except KeyError:
                    raise ValueError(f'label {param.label} not found')
                relative = position - instruction_counter
                relative = ctypes.c_uint(relative).value
                instr.params[i] = InstructionParam(ParameterType.IMMEDIATE_FOUR_BYTE, relative)

    for instr in tree:
        print(instr)


if __name__ == '__main__':
    main()
