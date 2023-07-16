 c
#include <inttypes.h>

int main() {
    uint64_t a = 42;
    __asm__ __volatile__ ( "push %0; pop %0" : "+g"(a) );
    printf("%016" PRIu64 "x\n", a);

    return 0;
}
