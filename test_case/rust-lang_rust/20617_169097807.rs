 rust
trait Foo {
    fn foo(&self, other: &Self) where Self: Sized;
}
