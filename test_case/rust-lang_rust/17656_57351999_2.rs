 rust
let mut hash = source;
hash ^= hash >> 33;
hash *= PRIME2;
hash ^= hash >> 29;
hash *= PRIME3;
hash ^= hash >> 32;
return hash;
