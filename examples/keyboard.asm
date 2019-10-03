mov 65536, %i1

loop:
    in 1, %i0
    je %i0, 'q', end
    je %i0, 8, backspace
    str1 %i0, %i1
    inc %i1
    jmp loop
end:
    halt

backspace:
    dec %i1
    str1 0, %i1
    jmp loop
