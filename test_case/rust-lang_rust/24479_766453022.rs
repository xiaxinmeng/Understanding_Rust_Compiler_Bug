rust
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        panic!()
    }
}

thread_local!(static FOO: Foo = Foo);

#[test]
pub fn test() {
    FOO.with(|_| {});
}
