 rust
#[deriving(Trace)]
enum Foo {
    X(Gc<int>, Vec<Gc<int>>)
    Y(int, int)
    Z(int, Gc<Vec<int>>)
}
