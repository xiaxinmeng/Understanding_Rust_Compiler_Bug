 c
  uint64_t a = (hash_result ^ seed) * kMul;
  a ^= (a >> 47);
  uint64_t b = (seed ^ a) * kMul;
  b ^= (b >> 47);
  b *= kMul;
  return b;
