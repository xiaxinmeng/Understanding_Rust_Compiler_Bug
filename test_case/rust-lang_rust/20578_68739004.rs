 rust
fn foo(mut f: Box<FnMut(int)>) {
    f(5)
}

fn bar(mut f: &mut FnMut(int)) {
    f(5)
}
