rust
fn bar<F: Fn()>(f: F) -> F {
    f
}

fn foo<F: Fn()>(x: bool, _: F) {
    if x {
        foo(false, bar(|| {}))
    }
}

fn main() {
    foo(true, bar(|| {}));
}
