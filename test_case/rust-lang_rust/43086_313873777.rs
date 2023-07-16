rust
// ok?
#[cfg(any(target_pointer_width="32", target_pointer_width="64"))]
impl ExactSizeIterator for RangeInclusive<u16> { ... }

// but this maybe bad idea
#[cfg(target_pointer_width="64")]
impl ExactSizeIterator for RangeInclusive<u32> { ... }
