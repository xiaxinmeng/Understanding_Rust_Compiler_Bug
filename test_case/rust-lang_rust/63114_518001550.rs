rust
#![feature(decl_macro)]

mod n {
    pub struct B(pub(crate) p::C);
    impl B {
        pub fn new() -> Self {
            B(p::C)
        }
    }
    mod p {
        pub struct C;

        impl C {
            pub fn foo(&self) -> i32 {
                33
            }
        }
    }
}

// This macro can (currently) be called cross-crate
pub macro m() {
    n::B::new().0.foo()
}
