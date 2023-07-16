rust
use std::ops::Index;
trait SubIndex<I: ?Sized>: Index<I> { }
