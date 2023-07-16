rust
use std::cell::RefCell;
use std::rc::Rc;

pub trait Trait {}

impl PartialEq<dyn Trait> for dyn Trait {
    fn eq(&self, _other: &Self) -> bool {
        todo!();
    }
}

pub fn eq(a: &Rc<RefCell<dyn Trait>>, b: &Rc<RefCell<dyn Trait>>) -> bool {
    *a == *b
}
