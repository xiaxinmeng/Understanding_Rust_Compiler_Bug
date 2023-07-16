rust
#![feature(const_generics)]

pub fn broke(const_g: u8) -> () {}

pub trait A<const CONST_GEN: u8> {
    fn call_broken() -> () {
        broke(CONST_GEN)
    }
}
// Cloning the const works. (if you need a temporary workaround)
//pub trait A<const CONST_GEN: u8> {
//    fn call_ok() -> () {
//        let x = CONST_GEN.clone();
//        broke(x)
//    }
//}
const CONST: u8 = 2;

struct Test {}

impl A<CONST> for Test {}

impl Test {
    fn call_broken_<const CONST: u8>() -> () {
        broke(CONST)
    }
}
fn call_ok<const CONST: u8>() -> () {
    broke(CONST)
}
fn main() {
    call_ok::<CONST>();
    Test::call_broken();
    Test::call_broken_::<CONST>();
}
