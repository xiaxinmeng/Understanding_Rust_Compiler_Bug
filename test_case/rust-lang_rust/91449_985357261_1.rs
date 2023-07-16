C
#include <stdint.h>

int sub(int8_t * x) {
    int8_t val = (int8_t) ((*x) - 10);
    if(val < 0){
        __builtin_unreachable();
    }
    return val;
}
