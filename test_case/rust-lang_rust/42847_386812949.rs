rust
fn null<T: ?Sized>() -> *const T
where
    T::Meta: Default,
{
    /* assemble a custom DST from { ptr = 0, meta = T::Meta::default() } */
}
