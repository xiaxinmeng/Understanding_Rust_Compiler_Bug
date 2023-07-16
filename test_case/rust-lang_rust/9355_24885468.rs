
trait Iterable<T> {
    fn iter(&self) -> Iterator<T>;
    fn map(&self, f: fn(&T) -> U) -> MapIterator<U> { self.iter().map(f) }
    ...
}

impl<T> Iterable<T> for Option<T> { ... }
