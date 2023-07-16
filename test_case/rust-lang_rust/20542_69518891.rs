 rust
use std::ops::Mul;
pub fn sqr<T>(x:T) -> Mul<T>::Output where T: Copy+Mul {
    x*x
}
