from emulator.machine_state import MachineState
from isa.instruction import Instruction


def halt(instr: Instruction, state: MachineState):
    assert len(instr.params) == 0
    state.running = False
