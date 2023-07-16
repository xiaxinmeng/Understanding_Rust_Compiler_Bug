rust
fn foo<'a>(x: &'a i32, f: &'static fn(&'a i32)) {
    f(x);
}

fn bar() {
    let x = 3;
    foo(&x, &(baz as _));
}

fn baz(_: &i32) {}
