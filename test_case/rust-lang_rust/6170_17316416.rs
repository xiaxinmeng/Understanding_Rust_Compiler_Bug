
#include "gpr.h"

#if defined(__APPLE__) && defined(__arm__)
#define LOAD(rn) do { \
    uintptr_t tmp; \
    asm("mov " #rn ",%0" : "=&r" (tmp) :); \
    this->rn = tmp; \
} while (0)
#else
...// as before..
#endif
