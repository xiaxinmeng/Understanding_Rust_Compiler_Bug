 rust
trait example_trait {}

fn main() {
    let x = 0u;
    let ptr = (&x as *const example_trait);
}
