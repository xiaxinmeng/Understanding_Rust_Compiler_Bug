rust
/// Helper for casting `&'static u32` into `usize`
union Union {
    a: &'static u32,
    b: usize,
}
