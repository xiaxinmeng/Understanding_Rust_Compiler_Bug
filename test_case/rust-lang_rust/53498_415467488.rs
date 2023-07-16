rust
pub mod test {
    pub struct A;
    pub struct B;
    pub struct Foo<T>(T);

    impl Foo<A> {
        fn foo() {}
    }

    impl Foo<B> {
        fn foobar() {}
    }
}

fn foo() {
    test::Foo::<test::A>::foobar();
}
