rust
trait DiscriminantKind {
    // the traits are required by `mem::Discriminant`
    type Discriminant: Clone + Copy + Debug + Eq + PartialEq + Hash + Send + Sync + Unpin;
}
