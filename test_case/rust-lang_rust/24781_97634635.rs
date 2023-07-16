 rust
use std::collections::Bound;
trait Bounds {
    fn start(&self) -> Bound<&T>;
    fn end(&self) -> Bound<&T>;
}
impl<T> Bounds for Range<T> {
    fn start(&self) -> Bound<&T> { Bound::Included(&self.start) }
    fn start(&self) -> Bound<&T> { Bound::Excluded(&self.end) }
}
// ...
