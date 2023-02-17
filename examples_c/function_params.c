#include "stdint.h"
#include "stdio.h"

int64_t test(int64_t a, int64_t b);

int main(void) {
    test(3, 2);
}

int64_t test(int64_t a, int64_t b) {
    int64_t c = a + 1;
    putcharln(c + '0');
    int64_t d = b + 2;
    putcharln(d + '0');
    int64_t e = c + d;
    return e;
}
