 Rust
trait Trait<T> {
    type A;
}

fn fails<T>() where T: Trait<T::A> {
}

fn main() {
    println!("Hello, world!");
}
