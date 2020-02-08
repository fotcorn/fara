#!/usr/bin/env bash
set -euxo pipefail
cargo run ../examples/add.bin
cargo run ../examples/hello_world.bin
cargo run ../examples/jump.bin
cargo run ../examples/loop.bin
cargo run ../examples/size.bin
cargo run ../examples/stack.bin
cargo run ../examples/subroutine.bin
