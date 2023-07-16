rust
// Current
trait Ord {
    fn clamp(self, min: Self, max: Self) -> Self { ... }
}
assert_eq!(9.clamp(6, 7), 7);


// Alternative
trait Ord {
    fn clamp(self, range: RangeInclusive<Self>) -> Self { ... }
}
assert_eq!(9.clamp(6..=7), 7);
