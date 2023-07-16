rust
macro_rules! some_macro {
    ($struct_name:ident) => {
        struct $struct_name<const N: usize>([usize; N]);
    }
}
some_macro!(Foo);
