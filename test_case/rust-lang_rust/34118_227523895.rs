 rust
trait MyTrait {}

impl<T: FnOnce()> MyTrait for T {}
impl<T> MyTrait for Rc<T> {}
