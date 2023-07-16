rust
use std::fmt::Debug;

struct Foo<T>(T);

unsafe impl<T> Send for Foo<T>
where T: Copy + Send + Clone + Debug {}
