from typing import List

from emulator.machine_state import MachineState
from isa.instruction import InstructionParam


def cp(params: List[InstructionParam], state: MachineState):
    assert len(params) == 2
    value = state.get_value(params[0])
    state.set_value(params[1], value)


def add(params: List[InstructionParam], state: MachineState):
    assert len(params) == 2
    value1 = state.get_value(params[0])
    value2 = state.get_value(params[1])
    result = value1 + value2
    state.set_value(params[1], result)
