rust
struct Foo;
impl !Send for Foo {}

fn use_foo(_: Foo) {}

let _: impl Send = || {
    let guard = Foo;
    use_foo(guard);
    yield;
};
