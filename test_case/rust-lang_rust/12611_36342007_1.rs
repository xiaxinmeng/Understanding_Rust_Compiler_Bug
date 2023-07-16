 rust
trait A {}

trait B: A {}

trait C {
    fn test<T: B>(&self) {}
}

impl C for  int {
    fn test<T: A>(&self) {}
}

fn main() { }
