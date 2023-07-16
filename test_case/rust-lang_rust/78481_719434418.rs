C
#include <stdbool.h>

typedef __uint128_t u128;

extern u128 sum_u128(u128 a, u128 b, u128 c, u128 d);
/*
{
    return a + b + c + d;
}
*/

#define ONE   (((u128) 1 << 64 ) + 1)
#define TWO   (((u128) 2 << 64 ) + 2)
#define THREE (((u128) 3 << 64 ) + 3)
#define FOUR  (((u128) 4 << 64 ) + 4)
#define TEN   (((u128) 10 << 64 ) + 10)

extern bool test() {
    u128 ret;
    ret = sum_u128(ONE, TWO, THREE, FOUR);
    return ret == TEN;
}
