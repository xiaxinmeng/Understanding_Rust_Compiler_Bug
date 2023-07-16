plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.85
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `const_ptr::<impl *const T>::byte_offset_from` is not yet stable as a const fn
    |
    |
748 | /                 (some_uninit.as_ref().unwrap() as *const mem::MaybeUninit<T>)
749 | |                     .byte_offset_from(&some_uninit as *const Option<mem::MaybeUninit<T>>)
    |
    = help: add `#![feature(const_pointer_byte_offsets)]` to the crate attributes to enable

error: could not compile `core` due to previous error
