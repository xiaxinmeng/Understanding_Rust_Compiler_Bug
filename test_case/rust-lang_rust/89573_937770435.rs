rust
struct Foo(usize);
struct Bar<'s>(&'s usize);

impl<'s> Bar<'s> {
    fn hello(&self) {
        println!("Hello {}", self.0);
    }
}

trait HardToGAT: 'static {
    fn hard_to_gat(&self, func: &dyn for<'s> Fn(&Bar<'s>));
}

impl HardToGAT for Foo {
    fn hard_to_gat(&self, func: &dyn for<'s> Fn(&Bar<'s>)) {
        let bar = Bar(&self.0);
        func(&bar);
    }
}

#[test]
fn test() {
    let foo = Foo(42);

    // works:
    foo.hard_to_gat(&|bar| {
        bar.hello();
    });

    // doesn't:
    foo.hard_to_gat(&Bar::hello);
}
