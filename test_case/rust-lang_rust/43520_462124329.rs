rust
fn foo<F: Fn()>(x: bool, _: F) {
    if x {
        foo(false, || {})
    }
}

fn main() {
    foo(true, || {});
}
