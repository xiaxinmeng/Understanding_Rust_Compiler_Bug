rust
struct Foo;
impl !Send for Foo {}

let _: impl Send = || {
    let guard = ManuallyDrop::new(Foo);
    yield;
};
