rust
#[derive(Copy, Clone)]
pub enum Two { First, Second }
use Two::*;

#[no_mangle]
pub fn two_valued(x: Two) -> Two {
    match x {
        First => First,
        Second => Second,
    }
}
