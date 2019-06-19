from isa.instruction import Instruction
from isa.instruction_set import InstructionType
from emulator.instructions import io, other


def dispatch(instruction: Instruction):
    if instruction.instruction_type == InstructionType.OUT:
        io.out(instruction.params)
    elif instruction.instruction_type == InstructionType.HALT:
        other.halt(instruction.params)
    else:
        raise ValueError('unknown or unimplemented instruction')
