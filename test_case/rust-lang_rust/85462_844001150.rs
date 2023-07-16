rust
fn check_is_really_fused(mut it: impl core::iter::FusedIterator) {
    for _ in &mut it {}
    assert!(it.next().is_none()); // fails
}

fn main() {
    check_is_really_fused("".matches(""));
}
