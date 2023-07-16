c
#include <stdio.h>
#include <string.h>

__thread char tb = 42;
__thread char zb32[32];  // LLVM opt adds `align 32`

int main()
{
    printf("%p %p\n", &tb, zb32);
    *(__m256 *)zb32 = _mm256_set_ps(0, 0, 0, 0, 0, 0, 0, 0);
}
