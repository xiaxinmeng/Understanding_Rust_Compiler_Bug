rust
pub trait Foo {}

pub trait Bar: !Foo {}

// We now know that the two traits are exclusive, thus can write:
trait X {}
impl<T: Foo> X for T {}
impl<T: Bar> X for T {}
