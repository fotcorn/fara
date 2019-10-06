
int_to_char: // parameter is in %i0
    mov 1, %i1  // i
    mov 10, %i2  // j
    loop1:
        je %i2, 1000000000, break1

        mov %i2, %i3
        mod %i0, %i3  // %i3 = x % j
        je %i3, %i0, break1

        inc %i1  // i++
        mul 10, %i2 // j = j * 10
        jmp loop1

    break1:
    mov %i1, %i0
