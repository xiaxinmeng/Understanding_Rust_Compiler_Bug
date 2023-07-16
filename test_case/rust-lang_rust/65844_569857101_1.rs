c
struct S { double x; double y; };

int indirect(struct S get_pair()) {
    struct S s = get_pair();
    return 0;
}
