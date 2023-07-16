 Rust
macro_rules! DECLARE_HANDLE {
    ($name:ident, $inner:ident) => {
        pub enum $inner { }
        pub type $name = *mut $inner;
    };
}
