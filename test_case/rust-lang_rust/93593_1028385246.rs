rust
trait Baz {
  const Bar: usize;
  const Qux: Self::Bar;
}
