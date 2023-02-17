#include "stdint.h"
#include "stdio.h"

int main(void) {
    int64_t a[3];
    a[0] = 5;
    a[1] = 3;
    a[2] = 2;

    for (int64_t i = 0; i < 3; i++) {
        putcharln('0' + a[i]);
    }
}
