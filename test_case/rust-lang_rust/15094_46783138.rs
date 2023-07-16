 rust
struct Adder<T> {
    x: T
}

impl<A, R, T: Add<A, R>> Adder<T> {
    fn call(&self, args: (A, )) -> R {
        let (y, ) = args;
        self.x + y
    }
}

fn make_adder<T>(x: T) -> Adder<T> {
    Adder {
        x: x
    }
}

pub fn main() {
    let add3 = make_adder(3i);
    let y: int = add3.call((17, ));
    println!("{}", y);
}
