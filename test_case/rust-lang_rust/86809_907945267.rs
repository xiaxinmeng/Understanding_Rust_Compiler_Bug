rust
pub enum VariantNonExhaustive {
    #[non_exhaustive]
    Bar {
        x: u32,
        y: u64,
    },
    Baz(u32, u16),
}

#[warn(non_exhaustive_reachable_patterns)]
match VariantNonExhaustive::Baz(1, 2) {
    VariantNonExhaustive::Baz(_, _) => {}
    VariantNonExhaustive::Bar { x, .. } => {}
}
