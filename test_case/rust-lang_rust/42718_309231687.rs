rust
fn foo<T>(_: T) {
    let _: fn() = || {};
}

fn main() {
    foo(0i8);
}
