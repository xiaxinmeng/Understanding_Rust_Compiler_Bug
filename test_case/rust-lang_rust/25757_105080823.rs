 rust
struct Foo {
    a: u32
}

impl Foo {
    fn x(&mut self) {
        self.a = 5;
    }
}

const FUNC: &'static FnMut(&mut Foo) -> () = &Foo::x;

fn main() {}
