import sys
from lark import Lark, Transformer, v_args
from instruction_set import Register, InstructionType
from instruction import InstructionParam, ParameterType, Instruction


GRAMMAR = '''start: (_statement NEWLINE)* _statement NEWLINE?

            number: INT
            address: "$"INT
            REGISTER_NUMBER: "0".."7"
            register: "%i"REGISTER_NUMBER
            ?param: (number|address|register)

            instruction: WORD (param (","param)*)?
            label: CNAME":"

            _statement: instruction|label

            %import common.WORD
            %import common.CNAME
            %import common.INT
            %import common.WS_INLINE
            %import common.NEWLINE
            %ignore WS_INLINE
         '''


class Label:
    def __init__(self, label):
        self.label = label

    def __str__(self):
        return f'{self.label}:'


class ASMTransformer(Transformer):
    @v_args(inline=True)
    def address(self, address):
        return InstructionParam(ParameterType.ADDRESS, int(address))

    @v_args(inline=True)
    def number(self, number):
        return InstructionParam(ParameterType.IMMEDIATE_EIGHT_BYTE, int(number))

    @v_args(inline=True)
    def register(self, register):
        return InstructionParam(ParameterType.REGISTER, Register(int(register) + 1))

    def instruction(self, args):
        instruction, *params = args
        instruction = instruction.value.upper()
        instruction_type = InstructionType[instruction]
        return Instruction(instruction_type, params)

    @v_args(inline=True)
    def label(self, label):
        return Label(label)

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
    for instr in transformer.transform(tree):
        print(instr)


if __name__ == '__main__':
    main()
