rust
trait FooAndBar: Eq + Ord {}
impl<T> FooAndBar for T where T: Eq + Ord {}

fn main() {
    let x: Box<FooAndBar> = unimplemented!();
}
