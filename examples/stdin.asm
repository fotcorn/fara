start:
    in 1, %i0
    out 1, %i0
    out 1, '\n'
    jne %i0, 'q', start
halt
