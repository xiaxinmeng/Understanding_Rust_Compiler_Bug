
enum Foo {
    A(usize),
}

fn main() {
    drop::<fn(usize)->Foo>(Foo::A);
}
