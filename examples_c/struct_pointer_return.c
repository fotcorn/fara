#include "stdint.h"
#include "stdio.h"

struct MyStruct {
    int64_t a;
    int64_t b;
};

struct MyStruct* func() {
    struct MyStruct s;
    s.a = 3;
    s.b = 5;
    return &s;
}

int main() {
    struct MyStruct* s = func();
    putcharln('0' + s->a);
    putcharln('0' + s->b);
}
