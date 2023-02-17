// not working right now, assembler does not know about the "in" instruction yet
.globl _start
_start:
    loop:
    in 1, %i0
    out %i0, 1
    jne %i0, 'q', _start
    halt
