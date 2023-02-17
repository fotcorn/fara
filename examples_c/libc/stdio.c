#include "stdio.h"

void putchar(int64_t c) {
    asm("out 1, %i0");
}

void putcharln(int64_t c) {
    asm("out 1, %i0");
    asm("out 1, '\n'");
}
