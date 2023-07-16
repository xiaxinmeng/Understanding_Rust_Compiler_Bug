rust
fn foo<T>(_: T) {}

fn bar<'a>(_: &'a i32) {
    let f = foo::<&'a i32>;
    is_static(f);
}

fn is_static<T: 'static>(_: T) {}
