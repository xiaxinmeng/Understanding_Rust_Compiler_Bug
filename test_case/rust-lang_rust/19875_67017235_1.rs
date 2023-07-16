 rust
trait Iterator<T> {
    // note: `T` is a "type constructor": lifetime -> concrete type
    fn next<'a>(&'a mut self) -> T<'a>;
}
