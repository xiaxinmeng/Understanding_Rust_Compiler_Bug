rust
impl<U: FieldName, T> MaybeInit<U, T> for Uninit<U, T> {
    #[inline(always)]
    fn get(self) -> Option<T> {
        None
    }
}

impl<U: FieldName, T> MaybeInit<U, (T,)> for T {
    #[inline(always)]
    fn get(self) -> Option<(T,)> {
        Some(Init::<U, (T,)>::get(self))
    }
}

impl<U: FieldName, T> Init<U, (T,)> for T {
    #[inline(always)]
    fn get(self) -> (T,) {
        (self,)
    }
}
