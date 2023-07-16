rust
use std::any::Any;

trait Trait {
    type Assoc;
}

impl<T: 'static> Trait for T {
    type Assoc = &'static T; 
}

fn implies_t_static<T>(t: T, x: <T as Trait>::Assoc) -> Box<dyn Any> {
    Box::new(t)
}
