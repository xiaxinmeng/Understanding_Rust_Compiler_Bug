rust
fn bar(x: usize) {
    assert_eq!(x, 42);
}

#[inline(always)]
fn saturating_mul(lhs: usize, rhs: usize) -> usize {
    match lhs.checked_mul(rhs) {
        Some(x) => x,
        None => usize::MAX,
    }
}

fn main() {
    bar(saturating_mul(1, 42));
}
