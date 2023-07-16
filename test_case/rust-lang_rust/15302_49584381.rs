 rust
pub trait InPlaceIterable<'a, T, It: Iterator<&'a mut T>> {
    fn in_place_mut_iter(&'a mut self) -> It;
}

/// Implementors of this trait must follow the documented invariants.
/// Any implementor of this trait will also need to implement
/// InPlaceIterable to actually be useful
pub trait InPlaceMappable {
    fn ensure_drop_no_elements(&mut self); // set_len(0)
}

// C_T is the "source" collection, e.g. Vec<T>
// C_U is the "destination" collection e.g. Vec<U>
pub struct PartialMapper<C_T, C_U> { ... }

impl<'a, T, U, T_It: Iterator<&'a mut T>,
        C_T: InPlaceIterable<'a, T, T_It> + InPlaceMappable,
        // C_U doesn't need InPlaceIterable because we only construct iterators from
        // C_T, and transmute the &mut T to &mut U as necessary.
        // Similarly it doesn't need InPlaceMappable
        C_U> PartialMapper<C_T, C_U> {
    pub fn new(source: C_T) -> PartialMapper<C_T, C_U> { ... }
    pub fn to_dest(self) -> C_U { ... }
    pub fn shift(&mut self) -> Option<T> { ... }
    pub fn push(&mut self, val: U) { ... }
}
