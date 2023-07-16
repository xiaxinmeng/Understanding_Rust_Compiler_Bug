 rust
trait Foo<T> {
    fn process(&mut self, T);
}

impl<T, F> Foo<T> for F where F: FnMut(T) {
    fn process(&mut self, data: T) {
        self(data)
    }
}

fn bar<F: Foo<String>>(mut foo: F) {
    foo.process("foobar".into());
}

fn main() {
    bar(|x| println!("{}", x));
}
