rust
trait Foo {}

trait Bar {}
impl<'a, T: Bar> Bar for &'a T {}

impl<T: Foo> Bar for T {}
