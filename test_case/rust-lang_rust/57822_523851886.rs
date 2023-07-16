C
int main() {
        struct Foobar {
                int i;
        };
        struct Foobar foobar = { .i = 1 };
        return foobar.i;
}
