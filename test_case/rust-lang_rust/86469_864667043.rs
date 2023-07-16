rust
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

pub fn main() {
    let y: *const Struct = core::ptr::null();
    core::hash::Hash::hash(&y, &mut H);
}
