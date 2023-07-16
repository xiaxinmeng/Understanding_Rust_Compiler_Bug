
thread_local!(static foo: Foo = Foo);

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        foo.with(|f| {
        });
    }
}

fn main() {
    let f = Foo;
}
