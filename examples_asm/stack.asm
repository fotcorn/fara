.globl _start
_start:
    mov 1, %i0
    mov 42, %i1
    mov 1337, %i2
    push %i0
    push %i2
    push %i1

    pop %i3
    pop %i4
    pop %i5

    jne 42, %i3, fail
    jne 1337, %i4, fail
    jne 1, %i5, fail

    out 1, 'O'
    out 1, 'K'
    out 1, '\n'
    halt

    fail:
    out 1, 'N'
    out 1, '\n'
    halt
