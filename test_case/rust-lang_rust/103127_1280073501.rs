plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `transmute_copy` is not yet stable as a const fn
    --> library/core/src/mem/maybe_uninit.rs:1303:18
     |
1303 |         unsafe { super::transmute_copy(&ManuallyDrop::new(self)) }
     |
     = help: add `#![feature(const_transmute_copy)]` to the crate attributes to enable

error: `transmute_copy` is not yet stable as a const fn
error: `transmute_copy` is not yet stable as a const fn
    --> library/core/src/mem/maybe_uninit.rs:1323:18
     |
1323 |         unsafe { super::transmute_copy(&ManuallyDrop::new(self)) }
     |
     = help: add `#![feature(const_transmute_copy)]` to the crate attributes to enable

error: could not compile `core` due to 2 previous errors
