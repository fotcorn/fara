import sys

from compiler.code_generator import generate_code
from compiler.parser import parse


def main():
    if len(sys.argv) != 2:
        print(f'{sys.argv[0]} <file.asm>')
        sys.exit(1)
    with open(sys.argv[1]) as f:
        code = f.read()

    tree = parse(code)
    code = generate_code(tree)
    
    out_filename = sys.argv[1].rsplit('.', 2)[0] + '.bin'
    with open(out_filename, 'wb') as f:
        f.write(code)


if __name__ == '__main__':
    main()
