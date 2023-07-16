rust
fn noalias(_: &mut u32, _: &mut u32) {}

pub fn use_foo(foo: &mut Foo) {
    let Foo { r1, r2 } = foo;
    noalias(r1, r2);
}
