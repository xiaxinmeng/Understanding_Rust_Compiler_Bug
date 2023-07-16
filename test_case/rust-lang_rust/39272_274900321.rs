rust
trait Foo where Self: Sized {
    // Foo may or may not have `std::marker::Sized`
    fn foo() -> Bar<Self>;
}
