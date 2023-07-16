rust
trait M {}

struct Node<T: ?Sized>(Box<T>);

impl <T: Clone + ?Sized> M for Node<T> {}

impl <T> M for Node<[T]> {}
