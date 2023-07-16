c++
#include <array>
#include <stdint.h>
#include <stdlib.h>

void foo(std::array<uint8_t, 25> &a) {
    for (size_t i = 0; i < a.size(); i++) {
        if (i % 5 > 0) a.at(i - 1) = 0;
        if (i % 5 < 4) a.at(i + 1) = 0;
    }
}
