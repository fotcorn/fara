from emulator.machine_state import MachineState
from isa.instruction import Instruction


def _arithmetic(instr: Instruction, state: MachineState, operation):
    params = instr.params
    assert len(params) == 2
    value1 = state.get_value(params[0], instr.size)
    value2 = state.get_value(params[1], instr.size)
    result = operation(value1, value2)
    state.set_value(params[1], result, instr.size)


def add(instr: Instruction, state: MachineState):
    _arithmetic(instr, state, lambda a, b: a + b)


def sub(instr: Instruction, state: MachineState):
    _arithmetic(instr, state, lambda a, b: a - b)


def mul(instr: Instruction, state: MachineState):
    _arithmetic(instr, state, lambda a, b: a * b)


def div(instr: Instruction, state: MachineState):
    _arithmetic(instr, state, lambda a, b: a // b)


def mod(instr: Instruction, state: MachineState):
    _arithmetic(instr, state, lambda a, b: a % b)


def or_instr(instr: Instruction, state: MachineState):
    _arithmetic(instr, state, lambda a, b: a | b)


def and_instr(instr: Instruction, state: MachineState):
    _arithmetic(instr, state, lambda a, b: a & b)


def xor(instr: Instruction, state: MachineState):
    _arithmetic(instr, state, lambda a, b: a ^ b)


def inc(instr: Instruction, state: MachineState):
    params = instr.params
    assert len(params) == 1
    value = state.get_value(params[0], instr.size)
    state.set_value(params[0], value + 1, instr.size)


def dec(instr: Instruction, state: MachineState):
    params = instr.params
    assert len(params) == 1
    value = state.get_value(params[0], instr.size)
    state.set_value(params[0], value - 1, instr.size)
