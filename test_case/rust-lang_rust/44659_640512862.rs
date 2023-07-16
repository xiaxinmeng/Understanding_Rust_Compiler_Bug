rust

trait Trait {}

impl<T> Trait for T where T: Trait {}

fn func<X: Trait>() {}

fn main() {
    func::<u32>();
}
