 rust
struct Data;

trait Foo {}
trait Bar {}

impl<T> Bar for T    where T: Foo {}
impl    Bar for Data {}
