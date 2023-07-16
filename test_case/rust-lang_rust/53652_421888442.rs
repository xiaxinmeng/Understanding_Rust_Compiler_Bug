rust
let (a, b) = bytes.split_at_mut(8);
b[..4].copy_from_slice(a[1..5]);
