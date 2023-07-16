rust
// works fine
#[non_exhaustive]
pub enum Bar {
    A,
    B(usize),
    C { field: Foo },
}

// does not work
pub enum VariantNonExhaustive {
    #[non_exhaustive]
    Bar { x: u32 },
    Baz(u32),
}
