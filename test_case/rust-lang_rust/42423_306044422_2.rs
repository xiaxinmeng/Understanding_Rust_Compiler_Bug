c
__attribute__((noinline))
float clamp(float x) {
    return fmin(fmax(x, 0), 1);
}
