rust
let [octobyte 0..8, rest @ ..] = byteslice;
let num = u64::from_le_bytes(octobyte);
