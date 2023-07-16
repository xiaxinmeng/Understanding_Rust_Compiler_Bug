c
bool always_false(void) {
    int* p = malloc(4);
    int q;
    return p < &q;
}
