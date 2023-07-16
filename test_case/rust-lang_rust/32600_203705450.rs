 rust
trait Foo<T> {
    fn process(&mut self, T);
}

impl<T, F> Foo<T> for F where F: FnMut(T) {
    fn process(&mut self, data: T) {
        self(data)
    }
}

fn bar<F: for<'a> Foo<&'a str>>(mut foo: F) {
    foo.process("foobar");
}

fn main() {
    bar(|x:&str| println!("{}", x));
}
