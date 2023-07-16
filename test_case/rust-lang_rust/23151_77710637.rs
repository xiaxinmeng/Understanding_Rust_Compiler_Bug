 rust
enum Foo<T> {
    A(T),
    B
}

fn main() {
    let x = Foo::A(14);
    let y = match x {
        Foo::A(value) => value,
        Foo::B        => 7,
    };
    println!("{}", y);
}
