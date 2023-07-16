rust
fn repro<'a>(mut foo: Box<Foo + 'a>) {
    let borrowed: &mut (Foo + 'a) = foo.borrow_mut();
}

fn repro2(mut foo: Box<Foo>) {
    let borrowed: &mut (Foo + 'static) = foo.borrow_mut();
}
