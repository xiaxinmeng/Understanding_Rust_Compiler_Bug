rust
pub macro m($inner_str:expr) {
    #[doc = $inner_str]
    struct S;
}

m!(stringify!(foo));
