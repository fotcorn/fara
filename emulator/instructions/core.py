import bitstruct

from emulator.machine_state import MachineState
from isa.instruction import Instruction
from isa.instruction_set import ParameterType, InstructionSize


def mov(instr: Instruction, state: MachineState):
    params = instr.params
    assert len(params) == 2
    assert params[1].parameter_type == ParameterType.REGISTER
    value = state.get_value(params[0], instr.size)
    state.set_value(params[1], value, instr.size)


def ld(instr: Instruction, state: MachineState):
    params = instr.params
    assert len(params) == 2
    size = instr.size
    address = state.get_value(params[0], InstructionSize.EIGHT_BYTE)
    if size == InstructionSize.ONE_BYTE:
        value = bitstruct.unpack('u8', state.memory[address:address+1])
    elif size == InstructionSize.TWO_BYTE:
        value = bitstruct.unpack('u16', state.memory[address:address+2])
    elif size == InstructionSize.FOUR_BYTE:
        value = bitstruct.unpack('u32', state.memory[address:address+4])
    elif size == InstructionSize.EIGHT_BYTE:
        value = bitstruct.unpack('u64', state.memory[address:address+8])
    else:
        raise ValueError('Unknown instruction size')
    state.set_value(params[1], value, instr.size)


def str_instr(instr: Instruction, state: MachineState):
    size = instr.size
    value = state.get_value(instr.params[0], size)
    address = state.get_value(instr.params[1], InstructionSize.EIGHT_BYTE)

    if size == InstructionSize.ONE_BYTE:
        state.memory[address:address + 1] = bitstruct.pack('u8', value)
    elif size == InstructionSize.TWO_BYTE:
        state.memory[address:address + 2] = bitstruct.pack('u16', value)
    elif size == InstructionSize.FOUR_BYTE:
        state.memory[address:address + 4] = bitstruct.pack('u32', value)
    elif size == InstructionSize.EIGHT_BYTE:
        state.memory[address:address + 8] = bitstruct.pack('u64', value)
    else:
        raise ValueError('Unknown instruction size')


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
