 rust
#![crate_type = "lib"]
#![feature(associated_types)]

use std::ops::Neg;

struct Complex<T>;

impl<T> Neg for Complex<T> where T: Neg<Output=T> {
    type Output = Complex<T>;

    fn neg(self) -> Complex<T> {
        loop {}
    }
}
