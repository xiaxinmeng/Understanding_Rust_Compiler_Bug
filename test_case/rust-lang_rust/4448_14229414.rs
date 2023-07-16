
fn foo<T: Owned>() {}
fn main() {
    foo::<&static/str>();
}
