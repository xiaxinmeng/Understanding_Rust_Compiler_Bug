c
void foo(__externref_t x) {
    &x;
    struct { __externref_t y; } z = { .y = x };
}
