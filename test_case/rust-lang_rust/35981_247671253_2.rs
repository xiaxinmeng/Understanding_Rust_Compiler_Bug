 c
#include <stdlib.h>
void f5(long* s, size_t len) {
    for (size_t i = 0; i < len; i++) {
        for (size_t j = 0; j <= i + 1; j++) {
            if (j >= len) {
                abort();
            }
        }
    }
}
