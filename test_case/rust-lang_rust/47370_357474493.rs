rust
struct Foo;

impl Clone for Foo {
    fn clone(&self) -> Self {
        println!("Foo::clone()");
        Foo
    }
}

fn main() {
    [Foo, Foo].iter().cloned().zip([0].into_iter()).collect::<Vec<_>>();
}
