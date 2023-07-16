rust
trait Trait<A>: Sized {
    fn call(self) {}
}

impl<T> Trait<&'static T> for T {}

fn main() {
    Trait::call(&String::new());
}
