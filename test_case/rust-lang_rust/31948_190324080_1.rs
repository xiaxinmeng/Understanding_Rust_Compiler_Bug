 rust
// bar.rs
pub struct Foo;

mod private {
    trait Bar {}
    impl Bar for ::Foo {}
}
