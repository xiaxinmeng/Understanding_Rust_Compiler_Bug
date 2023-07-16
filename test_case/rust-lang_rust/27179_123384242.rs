 rust
struct Foo<T>(T);

impl<T> Foo<T> {
    fn foo(&self) {
        println!("generic");
    }
}

impl Foo<()> {
    fn foo(&self) {
        println!("specific");
    }
}

fn main() {
    let f = Foo(());
    <::Foo<()>>::foo(&f);
}

