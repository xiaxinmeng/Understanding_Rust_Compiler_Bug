
use std::io;
trait B {
    fn f(&self);
}

trait T : B { }

impl<U: T> B for U {
    fn f(&self) { io::println("Hey, I'm a T!"); }
}

impl T for int { }

fn main() {
    let _br = @0 as @B;
}
