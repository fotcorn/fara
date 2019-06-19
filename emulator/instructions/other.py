from typing import List

from emulator.exceptions import HaltException
from isa.instruction import InstructionParam


def halt(params: List[InstructionParam]):
    assert len(params) == 0
    raise HaltException()
