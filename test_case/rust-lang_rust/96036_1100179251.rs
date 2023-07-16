rust
pub trait Something<T> { }
pub struct Whatever;
impl<T> Something<Whatever> for T {}
