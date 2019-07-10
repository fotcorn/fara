import sys
from typing import List

from isa.instruction import InstructionParam
from isa.instruction_set import ParameterType


def out(params: List[InstructionParam]):
    assert len(params) == 2
    assert params[0].parameter_type == ParameterType.IMMEDIATE_ONE_BYTE
    assert params[1].parameter_type == ParameterType.IMMEDIATE_ONE_BYTE

    port = params[0].value
    data = params[1].value

    if port == 1:
        sys.stdout.write(chr(data))
