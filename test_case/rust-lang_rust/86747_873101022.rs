rust
trait Placeholder {}
impl<T> Placeholder for T {}
pub fn bar(_: *mut dyn Placeholder) {}
