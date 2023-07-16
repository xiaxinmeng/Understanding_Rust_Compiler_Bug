rust
// in libcore or libstd:

#[allow(non_camel_case_types)]
pub mod primitives {
    mod details {
        pub type Bool = bool;
        pub type Char = char;
        // ...
    }
    pub type bool = details::Bool;
    pub type char = details::Char;
    // ...
}
