rust
trait Trait {
    type Assoc;
    fn test(_: ());
}

impl<'a, T> Trait for &'a T {
    type Assoc = ();
    fn test(_: Self::Assoc) {}
}
