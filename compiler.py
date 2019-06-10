import sys


def main():
    if len(sys.argv) != 2:
        print(f'{sys.argv[0]} <file.asm>')
        sys.exit(1)


if __name__ == '__main__':
    main()
