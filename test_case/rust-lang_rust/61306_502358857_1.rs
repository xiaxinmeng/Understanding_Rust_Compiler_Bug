c++
#include <stdio.h>
#include <stdint.h>

enum E1 : uint32_t {
        E1v1 = 1,
        E1v2 = 2,
        E1v3 = 4,
};

enum E2 : uint64_t {
        E2v1 = 1,
        E2v2 = 2,
        E2v3 = 4,
};

enum E3 : uint32_t {
        E3v1 = 1,
        E3v2 = 2,
        E3v3 = 4,
};

extern "C" void test(E1 val_e1, E2 val_e2, E3 val_e3, uint32_t val_u)
{
        printf("%d %d %d %d\n", static_cast<uint32_t>(val_e1),
                        static_cast<uint64_t>(val_e2),
                        static_cast<uint32_t>(val_e3),
                        val_u);
}
