rust

#![allow(order_dependent_trait_objects)]
trait Trait {}

impl Trait for dyn Send + Sync {}
impl Trait for dyn Sync + Send {}
fn assert_trait<T: Trait + ?Sized>() {}

fn main() {
    // compiles right now as we first evaluate `dyn Send + Sync + 'static`,
    // but store `dyn Send + Sync + 'erased` in the cache.
    assert_trait::<dyn Send + Sync>();
}
