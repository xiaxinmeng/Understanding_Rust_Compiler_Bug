rust
    pub unsafe fn align_to_mut<U>(&mut self) -> (&mut [T], &mut [U], &mut [T]) {
        // Note that most of this function will be constant-evaluated,
        if mem::size_of::<U>() == 0 || mem::size_of::<T>() == 0 {
            // handle ZSTs specially, which is – don't handle them at all.
            return (self, &mut [], &mut []);
        }

        // First, find at what point do we split between the first and 2nd slice. Easy with
        // ptr.align_offset.
        let ptr = self.as_ptr();
        let offset = crate::ptr::align_offset(ptr, mem::align_of::<U>());
        if offset > self.len() {
            (self, &mut [], &mut [])
        } else {
            let (left, rest) = self.split_at_mut(offset);
            // now `rest` is definitely aligned, so `from_raw_parts_mut` below is okay
            let (us_len, ts_len) = rest.align_to_offsets::<U>();
            let mut_ptr = rest.as_mut_ptr();
            (
                left,
                from_raw_parts_mut(mut_ptr as *mut U, us_len),
                from_raw_parts_mut(mut_ptr.add(rest.len() - ts_len), ts_len),
            )
        }
    }
