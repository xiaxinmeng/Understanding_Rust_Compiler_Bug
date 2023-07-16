c++
#include <string.h>
bool compare(const char* k) {
    return memcmp(k, "123", 3) == 0;
}
