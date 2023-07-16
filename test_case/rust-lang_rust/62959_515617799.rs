rust
fn foo<T: Clone>(arg: T) {}
fn bar<T: Clone>(arg: T) {
    foo(arg);
}
