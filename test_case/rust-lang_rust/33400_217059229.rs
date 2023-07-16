 rust
trait Trait {
    type A;
}

fn fails<T>() -> T where T: From<Vec<T>> {
    unimplemented!()
}

fn main() {
    println!("Hello, world!");
}
