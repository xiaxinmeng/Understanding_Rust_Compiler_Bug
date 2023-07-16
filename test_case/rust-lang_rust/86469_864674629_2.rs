rust
#![feature(ptr_metadata)]

use core::ptr::Pointee;
trait Trait {
    type Associated;
}
pub struct T;
impl Trait for T {
    type Associated = ();
}
pub struct Struct {
    _field: <T as Trait>::Associated,
}

pub fn main() {
    let _x: <Struct as Pointee>::Metadata;
}
