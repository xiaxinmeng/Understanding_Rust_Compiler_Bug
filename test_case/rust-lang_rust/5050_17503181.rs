 rust
trait Tr {
    fn foo(&self);
}

impl<'self> Tr for &'self int {
    fn foo(&self) {
        println("hello");
    }
}

fn foo<T: Tr>(a: &T) { a.foo(); }

fn main() {
    foo(&5);
}
