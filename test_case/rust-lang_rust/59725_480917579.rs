rust
struct Foo<T = ()> {
    x: T
}

fn main() {
    let x = Foo { x: Default::default() };
}
