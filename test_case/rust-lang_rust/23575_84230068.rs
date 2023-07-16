 rust
impl<F> Copy for PairFoo<F>
    where F: Foo,
          <F as Foo>::T: Copy
{
}
