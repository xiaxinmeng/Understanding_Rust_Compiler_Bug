 rust
pub enum Foo {
    A = 0,
    B = 1,
    CAPACITY = 2,
}

fn main() {
    let arr = [0; Foo::CAPACITY as usize];
}
