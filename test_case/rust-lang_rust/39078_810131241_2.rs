cpp
#include <stdint.h>
uint8_t foo(const __uint128_t x) {
    return (uint8_t)(x % 10);
}
