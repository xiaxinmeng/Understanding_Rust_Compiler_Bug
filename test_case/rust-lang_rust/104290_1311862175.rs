c++
#include <cstdint>

struct WithPadding {
    uint8_t a;
    uint16_t b;
};

void foo(WithPadding* p) {
    *p = {1, 2}; // Two `mov`s
}

void bar(WithPadding* p) {
    WithPadding t = {1, 2};
    *p = t; // One `mov`
}
