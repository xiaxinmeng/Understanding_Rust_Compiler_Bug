rust
// note: private trait 
trait RepeatSlice: Sized {
    fn repeat_slice(slice: &[Self], n: usize) -> Vec<Self>;
}

impl<T: Clone> RepeatSlice for T {
    default fn repeat_slice(slice: &[Self], n: usize) -> Vec<Self> {
        let mut res = Vec::with_capacity(n * slice.len());
        for _ in 0..n {
            res.extend_from_slice(slice);
        }
        res
    }
}

impl<T: Copy> RepeatSlice for T {
    fn repeat_slice(slice: &[Self], n: usize) -> Vec<Self> {
        // the original copy_nonoverlapping implementation
    }
}

impl<T> [T] {
    pub fn repeat(&self, n: usize) -> Vec<T> 
    where 
        T: Clone,
    {
        T::repeat_slice(self, n)
    }
}
