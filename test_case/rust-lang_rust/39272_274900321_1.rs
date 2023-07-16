rust
trait Foo {
    // Foo may or may not have `std::marker::Sized`
    fn foo() -> Bar<Self> where Self: Sized;
}
