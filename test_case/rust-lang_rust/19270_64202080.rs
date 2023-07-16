 rust
    pub fn partition(self, f: |&T| -> bool) -> (Vec<T>, Vec<T>) {
        if self.len() == 0 {
            return (self, vec!());
        }

        let mut ptr = self.ptr as *const _;
        let end = unsafe { ptr.offset(self.len as int) };

        let (mut left_len, mut left, mut right_len, mut right) = if f(&self[0]) {
            let left = Vec::with_capacity(self.len);
            unsafe { ptr::copy_nonoverlapping_memory(left.ptr, ptr, 1); }
            (1i, left, 0i, self)
        } else {
            let right = Vec::with_capacity(self.len);
            unsafe { ptr::copy_nonoverlapping_memory(right.ptr, ptr, 1); }
            (0i, self, 1i, right)
        };

        ptr = unsafe { ptr.offset(1) };
        while ptr != end {
            if f(unsafe { &*ptr }) {
                unsafe {
                    ptr::copy_nonoverlapping_memory(left.ptr.offset(left_len), ptr, 1);
                }
                left_len += 1;
            } else {
                unsafe {
                    ptr::copy_nonoverlapping_memory(right.ptr.offset(right_len), ptr, 1);
                }
                right_len += 1;
            }
            ptr = unsafe { ptr.offset(1) };
        }

        left.len  = left_len  as uint;
        right.len = right_len as uint;

        (left, right)
    }
