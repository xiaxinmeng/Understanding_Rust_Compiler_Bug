
struct foo {
    int a;

    int f(int x) const {
        if (x < 7) throw 42;
        return f(x);
    }
};

int main() {
    foo{}.f(0);
}
