#!/usr/bin/env bash
set -euxo pipefail
cargo run -- --loader elf ../examples/add.o
cargo run -- --loader elf ../examples/hello_world.o
cargo run -- --loader elf ../examples/jump.o
cargo run -- --loader elf ../examples/loop.o
cargo run -- --loader elf ../examples/size.o
cargo run -- --loader elf ../examples/stack.o
cargo run -- --loader elf ../examples/subroutine.o
