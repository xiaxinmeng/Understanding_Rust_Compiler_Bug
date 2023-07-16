 c
void g(int *a1, int *a2, size_t len1, size_t len2) {
    for (size_t i = 0; i < len1 && i < len2; ++i) {
        a1[i] = a2[i];
    }
}
