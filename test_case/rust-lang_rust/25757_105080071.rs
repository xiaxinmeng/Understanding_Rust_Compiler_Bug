 rust
macro_rules! get_method {
    ( $x: ident ) => {
        &Foo::$x
    };
}

struct Foo {
    a: u32
}

impl Foo {
    fn x(&mut self) {
        self.a = 5;
    }
}

const FUNC: &'static FnMut(&mut Foo) -> () = get_method!(x);

fn main() {}
