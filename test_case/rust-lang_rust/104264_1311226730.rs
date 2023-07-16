rust
impl<T: Clone> Clone for Nope<T> {
    #[inline]
    fn clone(&self) -> Nope<T> {
        Nope {
            _phantom: self._phantom.clone(),
        }
    }
}

impl<T: Copy> Copy for Nope<T> {}
