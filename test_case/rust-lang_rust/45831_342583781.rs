C
#include <stdio.h>
#include <stdlib.h>

void foo() {
    int *x = (int*)malloc(4);
    if (!x) return;
    *x = 0;

    int *y = (int*)malloc(4);
    if (!y) {
        free(x);
        return;
    }
    *y = 0;

    if (x == y) {
        printf("Equal!\n");
    }

    free(x);
    free(y);
}
