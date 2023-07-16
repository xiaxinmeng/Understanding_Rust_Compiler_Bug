rust
const F: Option<NonNull<i32>> = Some(unsafe { NonNull::new_unchecked(std::ptr::null_mut()) });
