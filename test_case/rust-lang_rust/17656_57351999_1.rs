 rust
let total_len = 8;
let mut h64 = self.seed + PRIME5 + total_len;

let mut k1: u64 = source * PRIME2;
k1 = rotl64(k1, 31);
k1 *= PRIME1;
h64 ^= k1;
h64 = rotl64(h64, 27) * PRIME1 + PRIME4;

h64 ^= h64 >> 33;
h64 *= PRIME2;
h64 ^= h64 >> 29;
h64 *= PRIME3;
h64 ^= h64 >> 32;
return h64;
