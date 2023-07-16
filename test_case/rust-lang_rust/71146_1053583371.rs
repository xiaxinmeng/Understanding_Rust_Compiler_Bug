rust
pub fn ptr_len<T>(ptr: &UnsafeCell<[T]>) -> usize {
   unsafe { (&*(ptr.get() as *const [()])).len() }
}
