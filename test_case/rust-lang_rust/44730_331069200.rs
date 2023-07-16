
macro_rules! never {
    ($inline:ident) => {
        #[inline($inline)]
        pub fn foo(x: u8) -> u8 { x * x}
    }
}
never!{never}
