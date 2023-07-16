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

struct H;
impl core::hash::Hasher for H {
    fn finish(&self) -> u64 {
        todo!()
    }
    fn write(&mut self, _: &[u8]) {
        todo!()
    }
}

fn panic<T>() -> T {
    panic!()
}

pub fn main() {
    use core::hash::Hash;
    let metadata: <Struct as Pointee>::Metadata = panic();
    metadata.hash(&mut H);
}
