 rust
use std::any::Void;
use std::intrinsics::transmute;

fn main() {
    let x: Void = unsafe { transmute(()) };
    match x {}
}
