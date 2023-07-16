 rust
trait Foo {
    type Bar;
    fn m() -> Self { .. }

    fn n() {
        Foo::Bar // OK
        Foo::m(); // error, need to infer self-type
    }
}
