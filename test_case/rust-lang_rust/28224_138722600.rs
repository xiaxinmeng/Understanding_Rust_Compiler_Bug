 C
#include <stdio.h>
#include <stdlib.h>

extern void je_mallctl(const char *, void *, size_t *, void *, size_t);

int main() {
    int valgrind = 6;
    size_t len = 1;
    je_mallctl("config.valgrind", &valgrind, &len, NULL, 0);
    printf("%d\n", valgrind);
}
