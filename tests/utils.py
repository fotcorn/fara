from compiler.main import compile_code
from emulator.loop import run
from emulator.machine_state import MachineState


def execute(code):
    code += '\nhalt'
    binary = compile_code(code)

    state = MachineState()
    state.memory[0x1000:0x1000+len(binary)] = binary
    state.pc = 0x1000

    run(state)

    return state
