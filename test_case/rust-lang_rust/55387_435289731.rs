rust
#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate(&mut self) {}
}

fn main() {
    let foo = Foo;
    let () = (|| {
        foo.mutate();
    })();
}
