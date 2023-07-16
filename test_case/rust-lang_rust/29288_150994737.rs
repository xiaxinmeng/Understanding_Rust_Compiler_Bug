 rust
#![feature(core_intrinsics)]
fn main() {
    unsafe { std::intrinsics::unchecked_sdiv((),()); }
}
