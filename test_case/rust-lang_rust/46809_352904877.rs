Rust
// This struct has a single niche - the dealigned pointer within. Check that
// it is null-pointer-optimized correctly
struct Interesting {
    packed: Packed<&'static ()>,
    other: u8
}

assert_eq!(mem::size_of::<Interesting>(), mem::size_of::<Option<Interesting>>());
