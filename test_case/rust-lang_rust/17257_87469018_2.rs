 rust
struct Foo<T: ?Sized> {
    x: u8,
    y: T,
}

fn main() {
    let foo: &Foo<[isize]> = &Foo { x: 1, y: [1, 2, 3] };
    match *foo {
        Foo { x, ref y } => {},
    }
}
