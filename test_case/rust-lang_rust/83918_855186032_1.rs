rust
let [a, b, c, d, e, f, g, h, rest @ ..] = byteslice;
let num = u64::from_le_bytes([a, b, c, d, e, f, g, h]);
