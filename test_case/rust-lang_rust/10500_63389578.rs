 c
#include<complex.h>

struct test {
    float x, y;
};
struct test s() {
    struct test ret = { 1.0, 2.0 };
    return ret;
}

float complex c() {
    return 1.0 + 2.0 * I;
}
