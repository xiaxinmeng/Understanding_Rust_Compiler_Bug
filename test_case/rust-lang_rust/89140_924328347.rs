rust
trait Trait<T: ?Sized> {}
trait MyInto<T: ?Sized> {}
struct Struct {}

impl<T> Trait<MyInto<T>> for Struct {}
