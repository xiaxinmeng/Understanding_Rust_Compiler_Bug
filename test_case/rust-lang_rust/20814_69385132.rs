 rust
#![feature(unsafe_destructor)]

use std::{ptr, mem};

struct NonOwningVec<T> {
    ptr: *mut T,
    len: usize,
}

#[unsafe_destructor]
impl<T> Drop for NonOwningVec<T> {
    fn drop(&mut self) {
        unsafe {
            let end = self.ptr.offset(self.len as isize);
            while self.ptr != end {
                ptr::read(self.ptr);
                self.ptr = self.ptr.offset(1);
            }
        }
    }
}

trait RemoveRange {
    fn remove_range(&mut self, from: usize, to: usize);
}

impl<T> RemoveRange for Vec<T> {
    fn remove_range(&mut self, from: usize, to: usize) {
        assert!(from <= to);
        assert!(to <= self.len());

        let ptr = self.as_mut_ptr();
        let len = self.len();

        unsafe {
            self.set_len(from);
            let middle = NonOwningVec { ptr: ptr.offset(from as isize), len: to  - from };
            let end    = NonOwningVec { ptr: ptr.offset(to   as isize), len: len - to   };
            drop(middle);
            mem::forget(end);
            ptr::copy_memory(ptr.offset(from as isize), ptr.offset(to as isize), len - to);
            self.set_len(len - (to - from));
        }
    }
}

impl RemoveRange for String {
    fn remove_range(&mut self, from: usize, to: usize) {
        assert!(self.is_char_boundary(from));
        assert!(self.is_char_boundary(to));

        unsafe { self.as_mut_vec().remove_range(from, to); }
    }
}
