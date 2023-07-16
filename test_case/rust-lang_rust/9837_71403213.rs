 rust
const S: i16 = 0x12345678;

enum E { V = S as isize }

fn main() { assert_eq!(S as u64, E::V as u64); }
