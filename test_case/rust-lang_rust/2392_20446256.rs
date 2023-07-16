
struct Foo {
    f: ~fn(),
}
fn foo(f: Foo) {
    f.f();
}
fn main() {}
