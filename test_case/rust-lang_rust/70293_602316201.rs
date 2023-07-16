rust
trait Foo {
    fn foo();
}

impl<T> Foo for T {
    #[track_caller]
    default fn foo() { panic!("default") }
}

impl Foo for () {
    fn foo() { panic!("final") }
}
