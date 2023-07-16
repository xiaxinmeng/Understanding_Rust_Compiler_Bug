Rust
struct Foo<T>(*const T);
impl<T: 'static> Copy for Foo<T> {}
impl<T: 'static> Clone for Foo<T> {
    fn clone(&self) -> Self { *self }
}

fn main() {
    let s = 2;
    let upvar = Foo(&|| s);
    let k = || upvar;
    k();
}
