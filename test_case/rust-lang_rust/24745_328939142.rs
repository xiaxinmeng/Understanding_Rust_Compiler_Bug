 rust
use std::fmt;
struct Bar;
trait LocalMarker {}
impl LocalMarker for Bar {}
impl<T: LocalMarker> fmt::Display for Vec<T> { } 
fn main() { }
