 rust
struct Foo<Sized? T> {
    x: u8,
    y: T,
}

fn main() {
    let foo: &Foo<[int]> = &Foo { x: 1, y: [1i, 2, 3] };
    match *foo {
        Foo { x, ref y } => {},
    }
}
