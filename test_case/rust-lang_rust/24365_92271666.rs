 rust
pub enum Foo {
    Test(Bar)
}

pub struct Bar {
    a: u32,
    b: u32
}

fn test(a: Vec<Foo>) {
    for lol in a {
        println!("test: {}", lol.b);
    }
}

fn main() { }

