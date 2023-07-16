rust
    fn into_val_slice_mut(mut self) -> &'a mut [V] {
        debug_assert!(!self.is_shared_root());
        unsafe {
            slice::from_raw_parts_mut(
                self.as_leaf_mut().vals.as_mut_ptr() as *mut V,
                self.len()
            )
        }
    }

    fn into_slices_mut(self) -> (&'a mut [K], &'a mut [V]) {
        let k = unsafe { ptr::read(&self) };
        (k.into_key_slice_mut(), self.into_val_slice_mut())
    }
