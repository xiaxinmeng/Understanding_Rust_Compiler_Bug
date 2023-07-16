 rust
use std::sync::RwLock;
use std::boxed::Box;

trait Trait {}
struct Thing;
impl Trait for Thing {}

fn main() {
  let _: Box<RwLock<Trait>> = Box::new(RwLock::new(Thing));
}
