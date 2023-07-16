rust
use std::ptr;

let mut v = Box::new(0i32);

unsafe {
    // Leaks the previously held value by overwriting the `Box<T>` with
    // a null pointer.
    ptr::write_bytes(&mut v as *mut Box<i32>, 0, 1);
}

let v2 = v; // UB! We are doing a typed copy of a `Box` that does not satisfy its validity invariant.
