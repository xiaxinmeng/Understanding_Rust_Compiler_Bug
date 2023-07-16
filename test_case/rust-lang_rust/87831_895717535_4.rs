rust
trait Collection {
    type Iter<'a>: IntoIterator
    where
        <Self::Iter<'a> as IntoIterator>::Item: std::fmt::Debug;
}
