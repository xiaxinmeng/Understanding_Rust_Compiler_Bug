rust
struct Foo {
    a: bool,
    b: str,
}

struct FooConcrete {
    a: bool,
    b: [u8; 10],
}

impl Drop for FooConcrete {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn coerce(a: Box<FooConcrete>) -> Box<Foo> {
    // ..
}

fn main() {
    let a = Box::new(FooConcrete {
        a: false,
        b: *b"helloworld",
    });
    let b = coerce(a);
    drop(b);
}
