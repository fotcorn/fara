from emulator.machine_state import MachineState
from isa.instruction import Instruction
from isa.instruction_set import InstructionType
from emulator.instructions import io, other, core


def dispatch(instruction: Instruction, state: MachineState):
    if instruction.instruction_type == InstructionType.CP:
        core.cp(instruction.params, state)
    elif instruction.instruction_type == InstructionType.ADD:
        core.add(instruction.params, state)
    elif instruction.instruction_type == InstructionType.OUT:
        io.out(instruction.params, state)
    elif instruction.instruction_type == InstructionType.HALT:
        other.halt(instruction.params)
    else:
        raise ValueError(f'unknown or unimplemented instruction: {instruction.instruction_type}')
