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


fn panic<T>() -> T {
    panic!()
}

pub fn main() {
    let metadata: <Struct as Pointee>::Metadata = panic();
    metadata == metadata;
}
