rust
#![feature(decl_macro)]

trait Trait {
    type T;
}

macro trait_impl {
    () => {
        type T = ();
    }
}

impl Trait for i32 {
    trait_impl!();
}
