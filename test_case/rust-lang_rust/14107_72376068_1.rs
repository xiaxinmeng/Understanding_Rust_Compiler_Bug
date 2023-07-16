 c
#include <string.h>
#include <unistd.h>

int main(void) {
    static char haystack[1024 * 1024];
    read(0, haystack, sizeof(haystack) - 1);

    const char *needle = "_";

    for (size_t i = 0; i < 10000000; i++) {
        size_t j = (size_t)strstr(haystack, needle);
        asm volatile("" : : "r"(j), "r"(&needle) : "memory");
    }
}
