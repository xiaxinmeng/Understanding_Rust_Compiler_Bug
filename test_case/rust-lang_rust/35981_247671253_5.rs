 c
#include <stdlib.h>
void check(unsigned int i, unsigned int len) {
    if (i < len) {
        if (i > (int)len - 1) {
            abort();
        }
    }
}
