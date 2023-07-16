rust
let (chunks, extra) = a.exact_chunks(16);
assert!(extra.is_empty(), "Have extra data");
for c in chunks {}
