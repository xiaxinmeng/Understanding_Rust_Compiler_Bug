rust
struct Foo<T> { x: T }

type Bar = Foo<u32>;

fn main() {
    let x = Bar { x: 22 };
}
