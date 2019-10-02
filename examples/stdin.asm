start:
    in 1, %i0
    str %i0, $65536
    jne %i0, 'q', start
halt
