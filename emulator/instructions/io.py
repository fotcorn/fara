import os
import sys
from typing import List

from emulator.machine_state import MachineState
from isa.instruction import InstructionParam


def out(params: List[InstructionParam], state: MachineState):
    assert len(params) == 2

    port = state.get_value(params[0])
    data = state.get_value(params[1])

    if port == 1:
        sys.stdout.write(chr(data))
    else:
        raise ValueError(f'Unsupported out port: {port}')


def in_instr(params: List[InstructionParam], state: MachineState):
    assert len(params) == 2
    port = state.get_value(params[0])

    if port == 1:
        while True:
            if len(state.stdin) > 0:
                char = state.stdin.pop()
                break
    else:
        raise ValueError(f'Unsupported in port: {port}')
    state.set_value(params[1], char)
