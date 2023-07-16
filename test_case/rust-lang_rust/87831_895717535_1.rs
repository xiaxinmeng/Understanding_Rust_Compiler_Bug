rust
trait Foo {
  type Bar<T> where Self::Bar<T>: Debug;
}
