rust
#[derive(Debug)]
struct Test<T> {
    a: T,
}
struct Undebugable;
fn main() {
    let a = Undebugable;
    let c = Test { a: a }; // <- ok
    println!("{:?}", c); // <- fails here
}
