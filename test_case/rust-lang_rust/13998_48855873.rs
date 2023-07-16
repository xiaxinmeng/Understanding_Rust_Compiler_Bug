 rust
pub struct Foo<T> {args: T}

impl<T> Foo<T> {
    pub fn new(args: T) -> Foo<T> {
        Foo {args: args}
    }

    pub fn connect(&mut self, slot: Bar<T>) {}
}

pub struct Bar<T> {pub listener: fn(&T)}

impl<T> Bar<T> {
    pub fn new(listener: fn(&T)) -> Bar<T> {
        Bar {listener: listener}
    }
}

fn two_args<T>(args: T) {
}

fn main() {
    let mut changed_args = Foo::new((2i));
    let f = Bar::new(two_args);

    changed_args.connect(f);
}
