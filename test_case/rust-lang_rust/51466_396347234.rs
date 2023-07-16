rust
impl<'b, T: ?Sized> Ref<'b, T> {
    fn map_split<U, V, F>(orig: Ref<'b, T>, f: F) -> (Ref<'b, U>, Ref<'b, V>)
    where
        U: ?Sized,
        V: ?Sized,
        F: FnOnce(&T) -> (&U, &V);
}

impl<'b, T: ?Sized> RefMut<'b, T> {
    fn map_split<U, V, F>(orig: RefMut<'b, T>, f: F) -> (RefMut<'b, U>, RefMut<'b, V>)
    where
        U: ?Sized,
        V: ?Sized,
        F: FnOnce(&mut T) -> (&mut U, &mut V);
}
