cp 0, %i0
loop:
    add 1, %i0
    cp %i0, %i1
    add 48, %i1
    out 1, %i1
    jne %i0, 5, loop
out 1, '\n'
halt
