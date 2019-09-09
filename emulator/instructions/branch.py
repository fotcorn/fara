from typing import List

from emulator.machine_state import MachineState
from isa.instruction import InstructionParam


def _compare(params: List[InstructionParam], state: MachineState, compare_func):
    assert len(params) == 3
    value1 = state.get_value(params[0])
    value2 = state.get_value(params[1])
    if compare_func(value1, value2):
        jump_target = state.get_value(params[2])
        state.pc += jump_target


def je(params: List[InstructionParam], state: MachineState):
    _compare(params, state, lambda a, b: a == b)


def jne(params: List[InstructionParam], state: MachineState):
    _compare(params, state, lambda a, b: a != b)


def jmp(params: List[InstructionParam], state: MachineState):
    assert len(params) == 1
    jump_target = state.get_value(params[0])
    state.pc += jump_target
