 Rust
trait Trait {
    type A;
}

fn fails<T: Trait>() -> T::A where T::A: From<Vec<<T as Trait>::A>> {
}

fn main() {
    println!("Hello, world!");
}
