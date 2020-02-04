#!/usr/bin/env bash
set -euxo pipefail

venv/bin/python compiler.py examples/hello_world.asm
venv/bin/python compiler.py examples/add.asm
venv/bin/python compiler.py examples/size.asm
venv/bin/python compiler.py examples/loop.asm
venv/bin/python compiler.py examples/jump.asm

venv/bin/python emulator.py examples/hello_world.bin
venv/bin/python emulator.py examples/add.bin
venv/bin/python emulator.py examples/size.bin
venv/bin/python emulator.py examples/loop.bin
venv/bin/python emulator.py examples/jump.bin
