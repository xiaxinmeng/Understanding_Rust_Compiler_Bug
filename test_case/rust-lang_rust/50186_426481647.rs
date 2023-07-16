rust
pub unsafe fn update_unsafe<F>(&self, f: F) -> T where F: FnOnce(T) -> T;
