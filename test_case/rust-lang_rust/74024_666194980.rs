rust
r = r.wrapping_mul(1664525).wrapping_add(1013904223);
let i = mapper(r % size);
black_box(v.binary_search(&i).is_ok());
