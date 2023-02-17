# FARA (FARA Research Architecture)
A custom Instruction Set Architecture (ISA) with a simulator written in Rust and a custom LLVM backend for code generation.

* 64bit instruction set with 24 general purpose integer register
* RISC architecture (no complex indirect adressing mode etc.)

Initial documentation is in the doc/ directory.

## LLVM backend
An LLVM backend for the ISA is developed here:
https://github.com/fotcorn/fara-llvm-backend

Current features of the backend:
* [x] Machine code generation from assembly
* [x] Instruction selection for simple C programs
* [x] Basic setup in Clang to support C programs
* [x] Linker support for ELF files in LLD
