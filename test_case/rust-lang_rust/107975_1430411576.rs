cpp
#include <stdio.h>
#include <cstdint>

int main(void) {
    int *a;
    int *b;

    {
        int v[2] = {0, 0};
        a = &(v[0]);
    }

    {
        int v[2] = {0, 0};
        b = &(v[0]);
    }

    printf("%p ==/^ %p is %d, %ld\n", a, b, a == b, ((uintptr_t)a) ^ ((uintptr_t)b));

    return 0;
}
