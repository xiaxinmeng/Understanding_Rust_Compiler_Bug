 c
void f(int * restrict a1, int * restrict a2, size_t len1, size_t len2) {
    for (size_t i = 0, j = 0; i < len1 && j < len2; ++i, ++j) {
        a1[i] = a2[j];
    }
}
