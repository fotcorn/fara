inc2:
    add 1, %i0
    add 1, %i0
    ret

.globl _start
_start:
    mov 5, %i0
    call inc2
    add 48, %i0
    out 1, %i0
    halt
