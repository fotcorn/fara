from typing import List

from emulator.machine_state import MachineState
from isa.instruction import InstructionParam


def halt(params: List[InstructionParam], state: MachineState):
    assert len(params) == 0
    state.running = False
