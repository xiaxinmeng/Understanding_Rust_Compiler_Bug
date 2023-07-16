rust
trait A {}
trait B {}

trait Foo { }

impl<T: A> Foo for T { }
impl<T: B> Foo for T { }
