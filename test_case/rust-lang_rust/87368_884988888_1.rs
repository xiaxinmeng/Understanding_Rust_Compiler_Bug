rust
#[repr(C, packed(8))]
struct FooBazAligned {
    bar: u64,
    baz: usize,
}
