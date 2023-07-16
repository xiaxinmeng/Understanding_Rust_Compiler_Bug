 rust
pub trait Ord<Rhs = Self>: Eq<Rhs> + PartialOrd<Rhs> {
    fn cmp(&self, other: &Rhs) -> Ordering;
}
