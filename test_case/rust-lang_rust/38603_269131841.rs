Rust
trait Q<T:?Sized> {}
trait Foo where u32: Q<Self> {}

fn main() {
    let f: Box<Foo> = ...;
}
