 rust
use std::intrinsics::TypeId;

fn main() {
    TypeId::of::<fn<'a>(&'a int)>();
}
