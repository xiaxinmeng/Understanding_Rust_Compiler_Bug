plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error: `MaybeUninit::<T>::assume_init_read` is not yet stable as a const fn
    |
    |
633 |             let value = new_box.assume_init_read();
    |
    = help: add `#![feature(const_maybe_uninit_assume_init_read)]` to the crate attributes to enable

error: could not compile `alloc` due to previous error
