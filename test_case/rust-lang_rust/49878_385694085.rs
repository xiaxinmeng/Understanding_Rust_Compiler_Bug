c
extern uint64_t add(uint64_t argc, va_list ap);

uint64_t test(uint64_t argc, ...) {
    uint64_t ret = 0;
    va_start(ap, argc);
    ret = add(ap);
    va_end(ap);
    return ret;
}
