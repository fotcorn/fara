from dataclasses import dataclass

@dataclass
class MachineState:
    i0: int = 0
    i1: int = 0
    i2: int = 0
    i3: int = 0
    i4: int = 0
    i5: int = 0

    pc: int = 0
    sp: int = 0

    memory: bytearray = bytearray(10000)


class Interpreter:
    def __init__(self):
        self.state = MachineState()


def main():
    state = MachineState()


if __name__ == '__main__':
    main()
