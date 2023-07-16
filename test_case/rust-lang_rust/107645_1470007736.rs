rust
struct Foo<T> {
    inner: T
}

impl<T> Foo<T> {
    fn into_inner(self) -> T { self.inner }
}

impl MyTrait for MyType {
    type Assoc = impl Trait;

    fn make_foo(foo: Foo<Self::Assoc>) -> Result<Self, MyError> {
        // this type annotation is really what defines Self::Assoc
        let inner: TheRealAssoc = foo.into_inner();
        // ...
    }
}
