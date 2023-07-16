rust
pub enum Provenance {
    Concrete {
        alloc_id: NonZeroU64,
        sb: NonZeroU64,
    },
    Wildcard,
}
