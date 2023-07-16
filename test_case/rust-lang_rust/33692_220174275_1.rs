
struct Foo;

impl Foo {
    fn get<T: Default>(&self) -> T {
        <T as Default>::default()
    }
}
