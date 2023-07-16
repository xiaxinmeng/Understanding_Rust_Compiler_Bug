cpp
#include <stdio.h>
#include <cstdint>

int main(void) {
    uintptr_t a;
    uintptr_t b;

    {
        int v[2] = {0, 0};
        a = (uintptr_t)&(v[0]);
    }

    {
        int v[2] = {0, 0};
        b = (uintptr_t)&(v[0]);
    }

    printf("%ld ==/^ %ld is %d, %ld\n", a, b, a == b, a ^ b);

    return 0;
}
