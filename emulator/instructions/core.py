from emulator.machine_state import MachineState
from isa.instruction import Instruction


def cp(instr: Instruction, state: MachineState):
    params = instr.params
    assert len(params) == 2
    value = state.get_value(params[0], instr.size)
    state.set_value(params[1], value, instr.size)


def add(instr: Instruction, state: MachineState):
    params = instr.params
    assert len(params) == 2
    value1 = state.get_value(params[0], instr.size)
    value2 = state.get_value(params[1], instr.size)
    result = value1 + value2
    state.set_value(params[1], result, instr.size)


def inc(instr: Instruction, state: MachineState):
    params = instr.params
    assert len(params) == 1
    value = state.get_value(params[0], instr.size)
    state.set_value(params[0], value + 1, instr.size)
