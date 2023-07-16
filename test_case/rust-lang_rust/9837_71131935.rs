
static S: isize = 0x12345678;

enum E { V = S }

fn main() { assert_eq!(S as u64, E::V as u64); }
