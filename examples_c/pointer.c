#include "stdint.h"
#include "stdio.h"

int main(void) {
    int64_t a = 5;
    int64_t* b = &a;
    *b = 6;
    putcharln('0' + a);
}
