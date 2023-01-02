#!/bin/bash
set -euo pipefail
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
set -x
$script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/add.asm -o $script_path/examples/add.o
# $script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/encoding.asm -o $script_path/examples/encoding.o
$script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/hello_world.asm -o $script_path/examples/hello_world.o
$script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/int_to_str.asm -o $script_path/examples/int_to_str.o
$script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/jump.asm -o $script_path/examples/jump.o
# $script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/keyboard.asm -o $script_path/examples/keyboard.o
$script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/loop.asm -o $script_path/examples/loop.o
$script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/size.asm -o $script_path/examples/size.o
$script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/stack.asm -o $script_path/examples/stack.o
# $script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/stdin.asm -o $script_path/examples/stdin.o
$script_path/../llvm-project/build/bin/clang  -c --target=fara $script_path/examples/subroutine.asm -o $script_path/examples/subroutine.o
