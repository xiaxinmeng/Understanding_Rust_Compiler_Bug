 rust
use std::sync::Arc;

trait TestTrait: Clone {}

pub struct Shared {
    pub thing: Arc<Box<TestTrait + Send + Share>>
}

fn clone<T: Clone>(t: &T) -> T { t.clone() }

impl Clone for Shared {
    fn clone(&self) -> Shared {
        Shared {
            thing: clone(&self.thing),
        }
    }
}
