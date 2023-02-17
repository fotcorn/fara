.globl _start
_start:
    out 1, 'a'  // 6
    jmp skip    // 8
    out 1, 'b'  // 6
    skip:       // =20
    out 1, 'c'
    out 1, '\n'
    halt
