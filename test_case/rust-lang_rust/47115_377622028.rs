rust
let (chunks, _) = a.exact_chunks(16);
for c in chunks {}
// or
for c in a.exact_chunks(16).0 {}
