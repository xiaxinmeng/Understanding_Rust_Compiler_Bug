 Rust
struct Foo<T> { x: T }
type Ty = Foo<u32>;
fn main() {
    match (Foo { x: () }) { Ty::<()> { x } => {} }
}
