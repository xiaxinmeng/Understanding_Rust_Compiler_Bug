rust
use std::fmt::Debug;

struct Foo;

trait Bar<T: Debug = Foo> {}
