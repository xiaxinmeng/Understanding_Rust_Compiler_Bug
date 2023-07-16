rust
#![feature(decl_macro)]

macro a() {
    pub struct A;

    mod module {
        use super::A; // Fails to resolve, since the the context of `A` is stripped when we try to resolve it in the root.
    }
}

a!();
