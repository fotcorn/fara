import sys

from emulator.machine_state import MachineState
from isa.instruction import Instruction
from isa.instruction_set import InstructionSize


def out(instr: Instruction, state: MachineState):
    assert len(instr.params) == 2

    port = state.get_value(instr.params[0], InstructionSize.ONE_BYTE)
    data = state.get_value(instr.params[1], InstructionSize.ONE_BYTE)

    if port == 1:
        sys.stdout.write(chr(data))
    else:
        raise ValueError(f'Unsupported out port: {port}')


def in_instr(instr: Instruction, state: MachineState):
    assert len(instr.params) == 2
    port = state.get_value(instr.params[0], InstructionSize.ONE_BYTE)

    if port == 1:
        while True:
            if len(state.stdin) > 0:
                char = state.stdin.pop()
                break
    else:
        raise ValueError(f'Unsupported in port: {port}')
    state.set_value(instr.params[1], char, InstructionSize.ONE_BYTE)
