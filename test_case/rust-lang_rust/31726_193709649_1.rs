
use std::u8; // module u8
use m::u16; // module u16

mod m {
    pub mod u16 {
        pub fn max_value() -> u16 { 33 }
    }
}

fn main() {
    type A = u8; // uses type u8, not the module in scope
    let a = u8::max_value(); // <type u8>::max_value
    let b = u8::MAX; // <mod u8>::MAX
    let c = u16::max_value(); // <mod u16>::max_value, not <type u16>::max_value
    assert_eq!(c, 33);
}
