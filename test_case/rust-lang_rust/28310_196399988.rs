 rust
#![crate_type = "lib"]

pub struct Dropper;

impl Drop for Dropper {
    fn drop(&mut self) {}
}

pub unsafe fn foo(d: Dropper) {
    std::mem::transmute::<_, u8>(d);
}
