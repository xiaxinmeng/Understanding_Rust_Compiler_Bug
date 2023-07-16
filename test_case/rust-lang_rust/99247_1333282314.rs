rust
trait Closure {}
impl<F> Closure for F where F: for<'a> FnOnce(&'a u32) {}

fn takes_closure(_: impl Closure) {}

fn main() {
    takes_closure(|_| ());
}
