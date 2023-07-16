
trait A { }
trait B: A { }
impl<T> B for T { }
fn main() {}
