 Rust
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

fn foo_foo<T>(t: &Foo<T>) { t.foo(); }

fn main() {
    let f = Foo(());
    foo_foo(&f);
}
