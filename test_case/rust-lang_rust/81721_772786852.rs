rust
iter.fold(
    0,
    #[rustc_inherit_overflow_checks]
    |a, b| a + b,
)
