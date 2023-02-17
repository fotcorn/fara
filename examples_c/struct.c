#include "stdint.h"
#include "stdio.h"

struct MyStruct {
    int64_t a;
    int64_t b;
};

int main() {
    struct MyStruct s;
    s.a = 1;
    s.b = 2;
    int64_t c = s.a + s.b;
    putcharln('0' + s.a);
    putcharln('0' + s.b);
    putcharln('0' + c);
}
