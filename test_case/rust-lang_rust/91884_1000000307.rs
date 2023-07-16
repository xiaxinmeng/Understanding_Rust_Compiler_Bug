plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error: `ptr::mut_ptr::<impl *mut T>::add` is not yet stable as a const fn
    |
    |
122 |             raw_ptr.add(old_size).write_bytes(0, new_size - old_size);
    |
    = help: add `#![feature(const_ptr_offset)]` to the crate attributes to enable

error: could not compile `alloc` due to previous error
