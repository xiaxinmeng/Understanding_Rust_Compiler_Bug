c++
#include <stdlib.h>
enum Foo {
    A,
    B
};

struct Vec {
    size_t len;
    Foo *data;
};

static int pop(Vec &v) {
    if (v.len) {
        return v.data[v.len-1];
    }
    return (int)Foo::B + 1;
}

Foo foo(Vec &v) {
    if (v.len == 0) { exit(1); }
    int k = pop(v);
    if (k > 1) {
        exit(3);
    }
    return (Foo)k;
}
