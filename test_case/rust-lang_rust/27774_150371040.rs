 C
struct foo {
    int a[16];
};
struct bar {
    int b;
};
void test(struct foo* foo, struct bar *bar) {
    for(int i=0; i<16; i++)
        foo->a[i] += bar->b;
}
