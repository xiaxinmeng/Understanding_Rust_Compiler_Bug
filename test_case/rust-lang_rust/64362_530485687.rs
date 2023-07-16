rust
#[derive(Debug)]
enum Foo {
    A { data: u32 },
}

impl Foo {
    fn new() -> Self {
        Self::A { data: 0 }
    }
}

fn main() {
    println!("created: {:?}", Foo::new());
}
