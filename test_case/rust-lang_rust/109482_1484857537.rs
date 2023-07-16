rust
use std::any::Any;

trait Trait: 'static {}

fn foo<T: Trait>(x: T) -> Box<dyn Any> {
    Box::new(x)
}
