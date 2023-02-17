#include "stdint.h"
#include "stdio.h"

int main(void) {
    for (int64_t i = 0; i != 5; i++) {
        putcharln('0' + i);
    }
}
