rust
use std::any::Any;

fn drain_dyn(v: &mut Vec<Box<dyn Any>>) -> impl Iterator<Item = Box<dyn Any>> {
    v.drain(..)
}
