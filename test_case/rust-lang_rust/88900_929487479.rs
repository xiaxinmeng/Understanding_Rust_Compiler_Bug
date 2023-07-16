rust
#[derive(Debug)]
struct Foo {
    #[allow(dead_code)]
    x: u32
}

fn main() {
    let _foo = Foo { x: 22 };
}
