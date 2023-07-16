rust
use std::rc::Rc;

pub fn foo() -> Rc<()> {
    Rc::new(())
}
