c
#include <stdint.h>
#include <stdio.h>
int main() {
    uint8_t arr[16];
    asm("" : "=m" (arr));
    printf("[%i, ..]\n", arr[0]);
    return 0;
}
