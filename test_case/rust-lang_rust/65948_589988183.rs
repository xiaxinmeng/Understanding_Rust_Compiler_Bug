rust
assert!(
    buf.chunks(2).all(|chunk| chunk[0] <= chunk[1]),
    "buffer is sorted",
);
