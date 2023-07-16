 rust
macro_rules! DECLARE_HANDLE {
    ($name:ident) => {
        with_concat_idents!($name, __, |inner| {
            pub enum inner { }
            pub type $name = *mut inner;
        });
    }
}
