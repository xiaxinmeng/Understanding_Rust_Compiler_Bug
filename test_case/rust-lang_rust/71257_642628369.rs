C++
#include <stdlib.h>
enum Foo {
    A,
    B,
    C,
};

Foo foo(Foo* data) {
    Foo k = data[1];
    if (k > Foo::C) {
        exit(3);
    }
    return k;
}
