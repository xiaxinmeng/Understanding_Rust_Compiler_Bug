 rust
macro_rules! make_foo {
    ($Int:ident) => (
        #[repr($Int)]
        enum Foo { A, B, C }

        impl Foo {
            fn min() -> Foo {
                unsafe {
                    ::std::mem::transmute::<$Int, Foo>(0)
                }
            }
        }
    )
}

make_foo!{ u8 }
