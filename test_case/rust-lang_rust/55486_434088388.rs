rust
trait Foo {}

impl<T> Foo for Vec<T> {}

trait Bar {}

impl<T> Foo for T
where
    T: Bar
{}
