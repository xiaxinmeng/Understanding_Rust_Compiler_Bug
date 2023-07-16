 rust
if end <= start {
    RangeInclusive::Empty { at: start }
} else {
    // Can't underflow as `end > start >= MIN`
    RangeInclusive::NonEmpty {
        start: start,
        end: end - 1,
}
