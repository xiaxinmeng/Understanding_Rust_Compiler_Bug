rust
// this currently does not generate add nuw instead of a simple add instruction,
// requires some future optimizations
a.checked_add(b).unwrap_or_else(|| core::hint::unreachable_unchecked())
