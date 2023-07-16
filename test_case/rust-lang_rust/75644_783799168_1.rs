rust
    pub fn try_generate<E, F>(mut cb: F) -> Result<Self, E>
    where
        F: FnMut(usize) -> Result<T, E>,
    {
        // SAFETY: we know for certain that this iterator will yield exactly `N`
        // items.
        unsafe {
            let array: [Result<T, E>; N] = collect_into_array_unchecked(&mut (0..N).map(cb));
            IntoIter::new(array).collect::<Result<Self, E>>()
        }
    }
