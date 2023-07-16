
struct Foo {
    a: i32,
}
type Bar = Foo;
fn baz(Bar { a }: Bar) {
}
