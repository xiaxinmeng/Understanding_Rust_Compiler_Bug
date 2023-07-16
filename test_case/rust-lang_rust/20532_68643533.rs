 rust
#![crate_type = "lib"]

use std::ops::Neg;

struct Complex<T>;

impl<T> Neg for Complex<T> where T: Neg {
    fn neg(self) -> Complex<T> {
        loop {}
    }
}
