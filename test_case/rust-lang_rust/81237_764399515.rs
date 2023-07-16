c
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

const size_t WIDTH = 64;
const size_t HEIGHT = 32;
const size_t DISPLAY_SIZE = WIDTH * HEIGHT;

bool draw(uint8_t x, uint8_t y) {
    x = (size_t)(x) % WIDTH;
    y = (size_t)(y) % HEIGHT;
    for (; y < HEIGHT; ++y)
    {
        size_t coord = x + y * WIDTH;
        if (coord >= DISPLAY_SIZE) {
            return false;
        }
    }
    return true;
}
