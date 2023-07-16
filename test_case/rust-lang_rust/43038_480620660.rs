rust
impl<T: ?Sized> Cell<T> {
    pub fn from_mut(t: &mut T) -> &Cell<T>;
}

impl<T> Cell<[T]> {
    pub fn as_slice_of_cells(&self) -> &[Cell<T>];
}
