rust
use std::mem;

pub trait NewDedup<T> {
    fn partition_dedup_new(&mut self) -> (&mut [T], &mut [T])
    where
        T: PartialEq<T>,
    {
        self.partition_dedup_by_new(|a, b| a == b)
    }

    fn partition_dedup_by_new<F>(&mut self, mut same_bucket: F) -> (&mut [T], &mut [T])
    where
        F: FnMut(&mut T, &mut T) -> bool;
}

impl<T> NewDedup<T> for &mut [T] {
    fn partition_dedup_by_new<F>(&mut self, mut same_bucket: F) -> (&mut [T], &mut [T])
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        let len = self.len();
        if len <= 1 {
            return (self, &mut []);
        }

        let ptr = self.as_mut_ptr();
        let mut next_read: usize = 1;
        let mut next_write: usize = 1;

        // SAFETY: the `while` condition guarantees `next_read` and `next_write`
        // are less than `len`, thus are inside `self`. `prev_ptr_write` points to
        // one element before `ptr_write`, but `next_write` starts at 1, so
        // `prev_ptr_write` is never less than 0 and is inside the slice.
        // This fulfils the requirements for dereferencing `ptr_read`, `prev_ptr_write`
        // and `ptr_write`, and for using `ptr.add(next_read)`, `ptr.add(next_write - 1)`
        // and `prev_ptr_write.offset(1)`.
        //
        // `next_write` is also incremented at most once per loop at most meaning
        // no element is skipped when it may need to be swapped.
        //
        // `ptr_read` and `prev_ptr_write` never point to the same element. This
        // is required for `&mut *ptr_read`, `&mut *prev_ptr_write` to be safe.
        // The explanation is simply that `next_read >= next_write` is always true,
        // thus `next_read > next_write - 1` is too.
        unsafe {
            // Avoid bounds checks by using raw pointers.
            while next_read < len {
                let ptr_read = ptr.add(next_read);
                let prev_ptr_write = ptr.add(next_write - 1);
                if !same_bucket(&mut *ptr_read, &mut *prev_ptr_write) {
                    next_write += 1;
                    next_read += 1;
                    // Avoid checking next_write != next_read once it is not in the same bucket,
                    // always swap memory if it is not in the same bucket.
                    while next_read < len {
                        let ptr_read = ptr.add(next_read);
                        let prev_ptr_write = ptr.add(next_write - 1);
                        if !same_bucket(&mut *ptr_read, &mut *prev_ptr_write) {
                            let ptr_write = prev_ptr_write.offset(1);
                            mem::swap(&mut *ptr_read, &mut *ptr_write);
                            next_write += 1;
                        }
                        next_read += 1;
                    }
                    return self.split_at_mut(next_write);
                }
                next_read += 1;
            }
        }

        self.split_at_mut(next_write)
    }
}
