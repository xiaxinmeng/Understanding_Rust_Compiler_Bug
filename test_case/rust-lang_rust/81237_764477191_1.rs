c
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <stdlib.h>

const size_t WIDTH = 64;
const size_t HEIGHT = 32;
const size_t DISPLAY_SIZE = WIDTH * HEIGHT;

bool draw(const uint8_t* data[DISPLAY_SIZE], const uint8_t x, const uint8_t y) {
    const size_t x2 = ((size_t)x) % WIDTH;
    const size_t y2 = ((size_t)y) % HEIGHT;
    bool collision = false;
    for (size_t y3 = y2; y3 < HEIGHT; y3++) {
        const size_t coord = x2 + y3 * WIDTH;
        if (coord < DISPLAY_SIZE) {
            collision |= data[coord] != 0;
        } else {
            exit(1);
        }
    }
    return collision;
}
