from emulator.machine_state import MachineState
from isa.instruction import Instruction
from isa.instruction_set import InstructionType
from emulator.instructions import io, other, core, branch


def dispatch(instruction: Instruction, state: MachineState):
    if instruction.instruction_type == InstructionType.MOV:
        core.mov(instruction, state)
    elif instruction.instruction_type == InstructionType.LD:
        core.ld(instruction, state)
    elif instruction.instruction_type == InstructionType.STR:
        core.str_instr(instruction, state)
    elif instruction.instruction_type == InstructionType.ADD:
        core.add(instruction, state)
    elif instruction.instruction_type == InstructionType.INC:
        core.inc(instruction, state)
    elif instruction.instruction_type == InstructionType.DEC:
        core.dec(instruction, state)
    elif instruction.instruction_type == InstructionType.OUT:
        io.out(instruction, state)
    elif instruction.instruction_type == InstructionType.IN:
        io.in_instr(instruction, state)
    elif instruction.instruction_type == InstructionType.HALT:
        other.halt(instruction, state)
    elif instruction.instruction_type == InstructionType.JMP:
        branch.jmp(instruction, state)
    elif instruction.instruction_type == InstructionType.JE:
        branch.je(instruction, state)
    elif instruction.instruction_type == InstructionType.JNE:
        branch.jne(instruction, state)
    else:
        raise ValueError(f'unknown or unimplemented instruction: {instruction.instruction_type}')
