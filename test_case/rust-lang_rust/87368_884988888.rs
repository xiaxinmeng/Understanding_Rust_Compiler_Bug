rust
#[repr(C, packed)]
struct FooBazUnAligned {
    bar: u64,
    baz: usize,
}
