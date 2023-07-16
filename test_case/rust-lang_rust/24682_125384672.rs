 rust
trait A: Sized {
    type N:One;
    fn new(x:Self::N) -> Self;

    fn x() -> Self {
        Self::new(Self::N::one())
    }
}

trait One {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> i32 {
        1i32
    }
}

impl A for B {
    type N = i32;
    fn new(x:i32) -> B {
        B { x: x }
    }
}

struct B {
    x:i32
}

fn main() {
    let b = B::new(5i32);
    println!("b.x: {}", b.x )
}
