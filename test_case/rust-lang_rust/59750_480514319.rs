rust
trait B {}

impl<C: FnOnce()> B for C {}

impl B for Box<dyn FnOnce()> {}
