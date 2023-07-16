rust
trait Foo {
    type Error;
    // It can be generic
    final type Result<T> = Result<T, Self::Error>;

    fn foo(&mut self) -> Self::Result<u32>;
}
