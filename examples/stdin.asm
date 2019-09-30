start:
    in 1, %i0
    cp %i0, $65536
    jne %i0, 'q', start
halt
