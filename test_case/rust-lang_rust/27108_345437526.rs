rust
// the 257 (&[u8], &[u8]) tuples here all hash the same way.
// if hashing uses 0xFF-termination instead of length prefix.
let data = [0xFFu8; 256];
let mut map = HashSet::new();
for i in 0..data.len() + 1 { map.insert(data.split_at(i)); }
