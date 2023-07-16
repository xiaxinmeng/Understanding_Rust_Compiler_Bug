rust
mod inner {
    #[derive(Default)]
    pub struct PubUnnameable;
}

trait Mirror { type Image; }
impl<T> Mirror for T { type Image = T; }

pub enum Pub<T> { Nothing, Just(T) }
pub trait Aux {}
impl Aux for <Pub<inner::PubUnnameable> as Mirror>::Image {}

pub fn assert_aux<T: Aux>(_t: &T) {}

impl inner::PubUnnameable {
    pub fn pub_method(self) {}
}
