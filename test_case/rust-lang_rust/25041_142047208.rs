
trait Foo {}

struct Bar<T> { _b: T }

impl<E, T: Unsize<E>> Foo for Bar<T> {}
