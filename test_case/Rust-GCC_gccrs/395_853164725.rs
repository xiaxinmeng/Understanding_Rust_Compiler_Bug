rust
trait Foo {
    fn bar();
}

struct Baz<T>(T)
where
    T: Foo;

struct Test<T>(T);

impl<T> Foo for Test<T> {
    fn bar() {}
}

fn main() {
    let a;
    a = Test(123);

    let b;
    b = Baz(a);
}

