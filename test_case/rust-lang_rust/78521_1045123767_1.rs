
struct foo {
    int a;

    int f(int x) const {
        return f(x);
    }
};

int main() {
    foo{}.f(0);
}
