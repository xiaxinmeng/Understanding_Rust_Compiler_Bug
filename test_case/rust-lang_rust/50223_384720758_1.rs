rust
#[repr(C)]
#[make_sure_trait_is_implemented]
union Value {
    a: u64,
    b: f64,
}
