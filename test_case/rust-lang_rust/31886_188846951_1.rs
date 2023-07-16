 rust
trait DiscriminantValue<T> {
    type Discriminant: Eq + Ord;
    fn value(&self) -> Self::Discriminant;
}
