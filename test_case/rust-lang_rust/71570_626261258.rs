rust
use std::rc::Rc;
trait LocalTrait {}
impl<F> LocalTrait for Rc<F> where F: Fn() {}
impl<F> LocalTrait for F where F: Fn() {}
