rust
#![feature(associated_type_defaults)]
#![feature(unboxed_closures)]

#[rustc_paren_sugar]
trait Tr<T> {
    type Output = u8;
    fn method() {}
}

impl<T> Tr<T> for u8 {}

fn main() {
    <u8 as Tr(&u8)>::method; // Sugared, ICE
    <u8 as Tr<(), Output = &u8>>::method; // Desugared, no ICE
}
