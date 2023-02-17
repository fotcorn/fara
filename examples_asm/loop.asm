.globl _start
_start:
    mov 0, %i0
    mov 5, %i2
    loop:
        add 1, %i0
        mov %i0, %i1
        add 48, %i1
        out 1, %i1
        jne %i0, %i2, loop
    out 1, '\n'
    halt
