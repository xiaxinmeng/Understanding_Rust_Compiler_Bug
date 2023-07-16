 c
#include <stdlib.h>
void check(unsigned int i, unsigned int len) {
    if (len >= 2) {
        if (i < len - 2) {
            if ((int)i + 2 > len) {
                abort();
            }
        }
    }
}
