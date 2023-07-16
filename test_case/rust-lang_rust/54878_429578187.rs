c
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

__attribute__((always_inline))
static inline void copy(int *restrict a, int *restrict b) {
    assert(a != b);
    *b = *a;
    *a = 7;
}

__attribute__((noinline))
void floppy(int mat[static 2], size_t idxs[static 3]) {
    for (int i = 0; i < 3; i++) {
        copy(&mat[i%2], &mat[idxs[i]]);
    }
}

int main() {
    int mat[3] = {10, 20};
    size_t idxs[3] = {1, 0, 1};
    floppy(mat, idxs);
    printf("%d %d\n", mat[0], mat[1]);
}
