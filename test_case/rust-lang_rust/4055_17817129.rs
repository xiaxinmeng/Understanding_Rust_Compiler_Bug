
trait A { }
trait B: A { }
impl<T: A> B for T { }
fn main() {}
