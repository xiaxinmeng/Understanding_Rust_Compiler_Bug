 rust
struct S<T> { v: T }

impl<T> S<T> {
    fn new(x: T) -> Self { S { v: x } }
}

type Alias<T> = S<Vec<T>>;

fn main() {
    // look at ma, no type annotations
    println!("{:?}", Alias::new(Some(4).iter().collect()).v);
}
