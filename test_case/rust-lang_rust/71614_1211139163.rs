rust
#![feature(decl_macro)]

macro trait_impl {
    () => {
        fn foo() {}
    }
}

struct Type;

impl Type {
    trait_impl!();
    fn foo() {}
}
