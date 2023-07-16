rust
trait Baz {
  const Bar: usize;
  const Qux: Self::Bar;
//           ^ Expected type, found const
}
