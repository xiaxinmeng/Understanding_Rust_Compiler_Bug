C
int foo(int a, ...) {
    va_list args;
    va_start(args, a);
    int b = va_arg(args, int);
    va_end(args);
    return a + b;
}
