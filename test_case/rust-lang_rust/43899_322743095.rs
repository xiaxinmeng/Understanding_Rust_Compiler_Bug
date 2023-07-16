
#include<inttypes.h>

#define alignof(type) (unsigned)((&((struct { char c; type d; } *)0)->d))

int main(void) {
    printf("%u %u %u %u\n", sizeof(uint64_t), alignof(uint64_t), sizeof(double), alignof(double));
    return 0;
}
