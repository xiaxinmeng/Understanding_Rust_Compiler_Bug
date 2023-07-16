rust
trait Iterable {
    type Item<'a>;
    type Iter<'a>: Iterator<Item = Self::Item<'a>>;
    
    fn iter<'a>(&'a self) -> Self::Iter<'a>;
}
