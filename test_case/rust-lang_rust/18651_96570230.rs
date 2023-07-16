 rust
trait Bar {
    fn frob(&self);
}

impl<'a, T> Bar for &'a [T] {
    fn frob(&self) {
        println!("Foo");
    }
}

fn main() {
    let x = Vec::new();
    x.frob(); // error: type `collections::vec::Vec<_>` does not implement any method in scope named `frob`
}
