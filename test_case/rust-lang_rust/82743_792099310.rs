
fn foo<T>() {
    fn bar(_: T) {}
}

fn baz<'a>() {
    fn buzz(_: &'a ()) {}
}
