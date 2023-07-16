rust
trait Trait {}
impl Trait for &() {}

fn get_trait<I>(a: &()) -> impl Trait {
    a
}
