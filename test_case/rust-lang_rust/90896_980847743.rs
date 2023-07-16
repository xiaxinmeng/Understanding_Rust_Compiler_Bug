plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: `std::mem::MaybeUninit::<T>::as_mut_ptr` is not yet stable as a const fn
    |
    |
292 |             *ptr::addr_of_mut!((*foo.as_mut_ptr()).x) = 1;
    |
    = help: add `#![feature(const_maybe_uninit_as_mut_ptr)]` to the crate attributes to enable


error: `std::mem::MaybeUninit::<T>::as_mut_ptr` is not yet stable as a const fn
    |
    |
298 |             *ptr::addr_of_mut!((*foo.as_mut_ptr()).y) = 2;
    |
    = help: add `#![feature(const_maybe_uninit_as_mut_ptr)]` to the crate attributes to enable

error: could not compile `core` due to 2 previous errors
