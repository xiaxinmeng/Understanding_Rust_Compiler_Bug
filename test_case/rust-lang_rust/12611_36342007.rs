 rust
trait A {}

trait B: A {}

trait C {
    fn test<T: A>(&self) {}
}

impl C for  int {
    fn test<T: B>(&self) {}
}

fn main() { }
