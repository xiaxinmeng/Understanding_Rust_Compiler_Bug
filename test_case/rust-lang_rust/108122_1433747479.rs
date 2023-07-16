rs
#[derive(Copy, Clone)]
#[repr(packed)]
pub struct Packed(i8, u32);

trait Foo {
    fn evil(&self);
}

#[automatically_derived]
impl Foo for Packed {
    fn evil(&self) {
        unsafe {
            &self.1;
        }
    }
}
fn main() {}
