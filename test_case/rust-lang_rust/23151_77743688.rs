 rust
enum Foo<T> {
    A(T),
    B
}

fn main() {
    let x = Foo::A::<u32>(14);
    let y = match x {
        Foo::A::<u32>(value) => value,
        Foo::B::<u32> => 7,
    };
    println!("{}", y);
}
