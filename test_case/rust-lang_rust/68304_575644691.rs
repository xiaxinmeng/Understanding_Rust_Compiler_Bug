c
int main(void) {
    int alignment = 16;
    __builtin_alloca_with_align(16, alignment);
}
