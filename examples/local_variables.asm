main:
    mov 5, %i0
    mov 3, %i1

    call add

    // convert to ascii
    add 48, %i0

    out 1, %i0
    out 1, '\n'

    halt

add:
    push %fp
    mov %sp, %fp
    sub 16, %sp

    strx %i0, %fp, -8
    strx %i1, %fp, -16

    ldx %fp, -8, %i2
    ldx %fp, -16, %i3

    add %i2, %i3

    mov %i3, %i0

    mov %fp, %sp
    pop %fp
    ret
