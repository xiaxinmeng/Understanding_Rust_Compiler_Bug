rust
fn foo<'a>(x: &'a i32, f: &'static fn(&'a i32)) {
    f(x);
}

fn bar<'a>(x: &'a i32) {
    foo(x, &(baz as fn(&'a i32)));
}

fn baz(_: &i32) {}
