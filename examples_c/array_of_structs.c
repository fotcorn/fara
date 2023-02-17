#include "stdint.h"
#include "stdio.h"

struct MyStruct {
    int64_t a;
    int64_t b;
};

int main(void) {
    struct MyStruct a[2];
    a[0].a = 2;
    a[1].a = 5;
    a[0].b = 3;
    a[1].b = 9;

    putcharln('0' + a[0].a);
    putcharln('0' + a[0].b);
    putcharln('0' + a[1].a);
    putcharln('0' + a[1].b);
}
