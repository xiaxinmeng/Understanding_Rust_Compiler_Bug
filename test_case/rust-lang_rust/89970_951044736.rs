rust
trait Iterable {
    type Item<'a>;

    type Iterator<'a>: Iterator<Item = Self::Item<'a>>;

    fn iter<'a>(&'a self) -> Self::Iterator<'a>;
}
