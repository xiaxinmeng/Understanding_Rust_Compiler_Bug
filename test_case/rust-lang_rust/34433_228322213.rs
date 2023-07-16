 rust
// Ranges of u64 and i64 are excluded because they cannot guarantee having
// a length <= usize::MAX, which is required by ExactSizeIterator.
range_exact_iter_impl!(usize u8 u16 u32 isize i8 i16 i32);
