#!/usr/bin/env bash
set -euxo pipefail

python compiler.py ../examples/add.asm
python compiler.py ../examples/hello_world.asm
python compiler.py ../examples/int_to_str.asm
python compiler.py ../examples/jump.asm
python compiler.py ../examples/keyboard.asm
python compiler.py ../examples/loop.asm
python compiler.py ../examples/size.asm
python compiler.py ../examples/stack.asm
python compiler.py ../examples/stdin.asm
python compiler.py ../examples/subroutine.asm
