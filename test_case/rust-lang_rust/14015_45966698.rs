 rust
trait Regexable<'a, T: Iterator<char>> {
    fn scannable(&self) -> MaybeIterator<'a, T>;
}
enum MaybeIterator<'a, T> {
    Iter(T),
    Slice(&'a str),
}
