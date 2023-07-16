rust
#![feature(core_intrinsics)]
use std::intrinsics::assume;

pub fn eat_digits(s: &[u8]) -> (&[u8], &[u8]) {
    let sep = s.iter().take_while(|x| (b'0'..=b'9').contains(x)).count();

    unsafe{ assume(sep <= s.len()); };

    (&s[..sep], &s[sep..])
}
