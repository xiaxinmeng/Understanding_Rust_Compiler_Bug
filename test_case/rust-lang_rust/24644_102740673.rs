 rust
trait Trait {}

struct Bar;
impl Trait for Bar {}

fn main() {
    let x: &[&Trait] = &[&Bar];
}
