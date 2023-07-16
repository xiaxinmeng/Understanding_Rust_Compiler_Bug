 rust
trait One: Sized {
    fn foo(self) { println!("One") }
}

trait Two {
    fn foo(&self) { println!("Two") }
}

impl One for i32 {}
impl Two for i32 {}

fn main() {
    42.foo();
}
