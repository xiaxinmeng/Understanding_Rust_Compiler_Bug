cpp
__uint128_t reversed(__uint128_t n) {
    __uint128_t reversed = 0;
    while (n != 0) {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    return reversed;
}
