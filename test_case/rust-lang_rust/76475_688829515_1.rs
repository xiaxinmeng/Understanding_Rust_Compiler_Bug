cpp
#include <stdint.h>
int main() {
    uint64_t k = 1'000'000'000;
    enum { N = sizeof(k) * 8 };
    return 0;
}
