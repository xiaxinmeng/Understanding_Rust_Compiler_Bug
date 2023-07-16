rust
pub fn swap_remove(&mut self, index: usize) -> T {
    unsafe {
        let hole: *mut T = &mut self[index];
        std::ptr::replace(hole, self.pop().unwrap())
    }
}
