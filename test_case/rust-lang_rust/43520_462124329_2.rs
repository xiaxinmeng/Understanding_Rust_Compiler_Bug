rust
fn dummy() {}

fn foo<F: Fn()>(x: bool, _: F) {
    if x {
        foo(false, dummy)
    }
}
