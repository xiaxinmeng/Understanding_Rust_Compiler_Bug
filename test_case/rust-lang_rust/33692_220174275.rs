 compile_fail
struct Foo;

impl<T: Default> Foo {
    fn get(&self) -> T {
        <T as Default>::default()
    }
}
