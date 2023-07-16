 rust
trait Visitor<R> {
    fn foo();
}

fn visit<V: Visitor<R>, R>() {
    V::foo()
}

struct V;

impl Visitor<()> for V {
    fn foo() { println!("hi") }
}

fn main() {
    visit::<V, ()>()
}
