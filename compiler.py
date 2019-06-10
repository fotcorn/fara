import sys
from lark import Lark

GRAMMAR = '''start: (instruction NEWLINE)* instruction NEWLINE?

            number: INT
            address: "$"INT
            register: "%i""0".."7"
            ?param: (number|address|register)
            instruction: WORD (param (","param)*)?

            %import common.WORD
            %import common.INT
            %import common.WS_INLINE
            %import common.NEWLINE
            %ignore WS_INLINE
         '''


def main():
    if len(sys.argv) != 2:
        print(f'{sys.argv[0]} <file.asm>')
        sys.exit(1)
    with open(sys.argv[1]) as f:
        code = f.read()

    lark = Lark(GRAMMAR)
    tree = lark.parse(code)
    print(tree.pretty())


if __name__ == '__main__':
    main()
