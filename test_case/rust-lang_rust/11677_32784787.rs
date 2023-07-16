 rust
trait X<T> {}

struct S<T> {f: ~X<T>, g: ~X<T>}

struct F;
impl X<int> for F {}

fn main() {
  S {f: ~F, g: ~F};
}
