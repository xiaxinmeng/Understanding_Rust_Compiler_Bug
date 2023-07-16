rust
    /// Reserve space for `len` more elements in the vector,
    /// and return a slice to the uninitialized tail of the vector
    fn reserve_get_tail_slice(vec: &mut Vec<T>, len: usize) -> &mut [MaybeUninit<T>] {
        // Reserve the new space.
        vec.reserve(len);

        // TODO: use `Vec::spare_capacity_mut` instead
        // SAFETY: `MaybeUninit<T>` is guaranteed to have the same layout
        // as `T`, and we already made sure to have the additional space.
        let start = vec.len();
        let tail_ptr = vec[start..].as_mut_ptr() as *mut MaybeUninit<T>;
        unsafe { slice::from_raw_parts_mut(tail_ptr, len) }
    }
