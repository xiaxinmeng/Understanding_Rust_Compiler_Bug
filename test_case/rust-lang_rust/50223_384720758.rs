rust
#[repr(C)]
#[derive(MakeSureTraitIsImplemented)]
union Value {
    a: u64,
    b: f64,
}
