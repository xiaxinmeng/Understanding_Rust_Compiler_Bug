rust
extern crate core;
fn main() {
    unsafe { std::intrinsics::copy(&0, &mut 0, 1); }
    core::num::Float::is_nan(1.0);
}