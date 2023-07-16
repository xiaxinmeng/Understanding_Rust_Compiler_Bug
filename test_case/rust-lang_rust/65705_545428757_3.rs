rust
unsafe fn transmute<U>(self) -> Vec<U> {
    let (ptr, len, cap) = self.into_raw_parts();
    let ptr = mem::transmute::<*mut T, *mut U>(ptr);
    Vec::from_raw_parts(ptr, len, cap)
}
