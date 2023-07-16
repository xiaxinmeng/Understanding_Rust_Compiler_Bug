
struct Range<T> { from: T, to: T }
impl<T:ToPrimitive> Iterator<T> for Range<T> { ... }
impl<T:-ToPrimitive> Iterator<T> for Range<T> { ... }
