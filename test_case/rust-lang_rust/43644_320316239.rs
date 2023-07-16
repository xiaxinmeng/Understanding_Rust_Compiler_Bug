rust
pub fn do_something<'a, F>(&'a self, _foo: F) where FooRef<'a>: From<F> {
	let _test_foo = <FooRef as From<&Foo>>::from(self);
}
