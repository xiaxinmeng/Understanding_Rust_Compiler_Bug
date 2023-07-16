
enum Foo {
    A { a: int, aa: int },
    B { b: int, bb: int },
}

macro_rules! thingy2(
    () => (
        match B { b: 10, bb: 10 } {
            A { a , aa} => 13,
            B { b , bb } => 14
        }
    )
)

fn main() {
    println!("{}",thingy2!());
}
