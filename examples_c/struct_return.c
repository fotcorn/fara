#include "stdint.h"
#include "stdio.h"

struct MyStruct {
    int64_t a;
    int64_t b;
    int64_t c;
};

struct MyStruct func() {
    struct MyStruct s;
    s.a = 2;
    s.b = 3;
    s.c = 4;
    return s;
}

int main() {
    struct MyStruct s = func();
    putcharln('0' + s.a);
    putcharln('0' + s.b);
    putcharln('0' + s.c);
}
