rust
trait Example: Clone {}
impl<T> Example for T where T: Clone { }
impl<T> Example for Vec<T> where T: Clone { }
