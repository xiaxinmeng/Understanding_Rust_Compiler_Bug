c++
#include <array>

void swap(std::array<char, 2048>&__restrict__ x, std::array<char, 2048>&__restrict__ y) {
    std::swap(x, y);
}
