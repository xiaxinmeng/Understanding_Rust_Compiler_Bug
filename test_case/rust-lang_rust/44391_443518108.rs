
#include <stdint.h>

_declspec( dllimport ) thread_local size_t a;

size_t test() {
    return a;
}
