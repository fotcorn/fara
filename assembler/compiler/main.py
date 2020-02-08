import sys

from compiler.code_generator import generate_code
from compiler.parser import parse


def compile_code(source_code):
    tree = parse(source_code)
    return generate_code(tree)


def main():
    if len(sys.argv) != 2:
        print(f'{sys.argv[0]} <file.asm>')
        sys.exit(1)
    if not sys.argv[1].endswith('.asm'):
        print(f'Input files need a .asm file extensions')
        sys.exit(1)

    with open(sys.argv[1]) as f:
        source_code = f.read()

    binary = compile_code(source_code)

    out_filename = sys.argv[1].rsplit('.', 1)[0] + '.bin'
    with open(out_filename, 'wb') as f:
        f.write(binary)
