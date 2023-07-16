 c
#include <stdio.h>

#define typename(x) _Generic((x), \
    char *const *:      "char *const *", \
    default:            "unreachable!()")

void f(char *const x[]) {
    puts(typename(x));
}

int main(void) {
    f(0);
}
