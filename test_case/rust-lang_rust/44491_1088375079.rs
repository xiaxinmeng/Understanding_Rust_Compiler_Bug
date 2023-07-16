rust
impl_scope! {
    struct Foo<T: Debug> { .. }

    // expands to impl<T: Debug> Self { .. }
    impl Self { .. }

    // where bounds supported too:
    impl Self where T: PartialEq { .. }
}
