
enum Flags {
    Flag1 = 1 << 0,
    Flag2 = 1 << 1,
    Flag3 = 1 << 2,
};
bool hasFlag(unsigned u, Flags f) {
     return !!(u & f);
}
