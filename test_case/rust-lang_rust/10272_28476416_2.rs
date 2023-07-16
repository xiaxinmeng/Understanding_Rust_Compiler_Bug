
enum Flags { Flag1, Flag2, Flag3 };
#define TO_BIT(F) (1 << (F))
bool hasFlag(unsigned u, Flags f) {
     return !!(u & TO_BIT(f));
}
