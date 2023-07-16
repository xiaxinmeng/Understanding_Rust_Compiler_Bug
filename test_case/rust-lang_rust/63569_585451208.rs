
impl<T> [MaybeUninit<T>] {
    pub fn first_ptr(&self) -> *const T { ... }
    pub fn first_ptr_mut(&mut self) -> *mut T { ... }
}
