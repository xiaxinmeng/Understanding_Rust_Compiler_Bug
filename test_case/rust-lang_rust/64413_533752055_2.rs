c
#include <stdio.h>

int main() {
    fprintf(stderr, "partial ");
    fprintf(stderr, "line\n");
    fprintf(stderr, "a one and a two and a %d and a %f\n", 3, 4.0);
    return 0;
}
