rust
#[repr(align(32))]
struct AlignedU64Slice<const N: usize>([u64; N]);
