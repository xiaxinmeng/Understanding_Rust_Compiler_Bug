 c
static void foo() {}

int main() {
    asm("call foo");
    return 0;
}
