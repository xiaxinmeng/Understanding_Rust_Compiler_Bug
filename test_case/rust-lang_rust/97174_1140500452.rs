cuda
#include <stdint.h>
#include <stdio.h>

struct foo {
    uint8_t a;
    uint8_t b;
    uint8_t c;
};

__attribute__((noinline)) __device__ struct foo device(uint8_t v) {
    struct foo s = {
        .a = v,
        .b = v,
        .c = v
    };
    return s;
}

extern "C" __global__  void kernel(struct foo* output, uint8_t const* input) {
    *output = device(*input);
}

