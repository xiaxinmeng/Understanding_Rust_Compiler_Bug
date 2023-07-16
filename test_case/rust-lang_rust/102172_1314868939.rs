rust
trait Trait {}

fn main() {
    let i = Box::new(1) as Box<dyn* Trait>;
}
