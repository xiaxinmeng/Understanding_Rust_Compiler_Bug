c
typedef struct {
  unsigned __int128 a;
  unsigned __int128 b;
} UInt128Pair;

UInt128Pair divmod(unsigned __int128 a, unsigned __int128 b) {
  UInt128Pair result;
  result.a = a / b;
  result.b = a % b;
  return result;
}
