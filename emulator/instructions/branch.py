import bitstruct

from emulator.machine_state import MachineState
from isa.instruction import Instruction


def _compare(instr: Instruction, state: MachineState, compare_func):
    params = instr.params
    assert len(params) == 3
    value1 = state.get_value(params[0], instr.size)
    value2 = state.get_value(params[1], instr.size)
    if compare_func(value1, value2):
        jump_target = state.get_value(params[2], instr.size)
        unsigned_int = bitstruct.pack('u32', jump_target)
        signed_int = bitstruct.unpack('s32', unsigned_int)[0]
        state.pc += signed_int


def je(instr: Instruction, state: MachineState):
    _compare(instr, state, lambda a, b: a == b)


def jne(instr: Instruction, state: MachineState):
    _compare(instr, state, lambda a, b: a != b)


def jmp(instr: Instruction, state: MachineState):
    assert len(instr.params) == 1
    jump_target = state.get_value(instr.params[0], instr.size)
    unsigned_int = bitstruct.pack('u32', jump_target)
    signed_int = bitstruct.unpack('s32', unsigned_int)[0]
    state.pc += signed_int
