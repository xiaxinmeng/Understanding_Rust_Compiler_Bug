
fn clone(&self) -> Self {
    unsafe {
        let result: Self = mem::uninitialized();
        clone_slice_into_uninitialized_memory(self.begin(), self.len(), result.begin());
        result
    }
}
