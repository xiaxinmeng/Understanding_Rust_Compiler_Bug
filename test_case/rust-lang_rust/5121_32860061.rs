 rust
trait Iterable<T, I: Iterator<T>> {
    fn iter(self) -> I;
}
impl<'a, T> Iterable<T, FooIter<'a, T>> for &'a Foo<T> {...}
// This impl wouldn't work with &'a self without DST.
impl<'a, T> Iterable<T, VecItems<'a, T>> for &'a [T] {...}
