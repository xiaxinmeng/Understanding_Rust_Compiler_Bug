
/// Returns the size of a type
pub fn size_of<T>() -> uint {
    TypeInfo::size_of(None::<T>)
}
trait TypeInfo {
    fn size_of(x: Option<Self>) { ... }
    fn size_of_val(&self) -> uint { ... }
}
