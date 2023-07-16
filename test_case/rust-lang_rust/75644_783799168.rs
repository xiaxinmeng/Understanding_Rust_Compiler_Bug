rust
    pub fn generate<F>(mut cb: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        // SAFETY: we know for certain that this iterator will yield exactly `N`
        // items.
        unsafe { collect_into_array_unchecked(&mut (0..N).map(cb)) }
    }
