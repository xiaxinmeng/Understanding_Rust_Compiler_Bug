plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `maybe_uninit::MaybeUninit::<T>::as_mut_ptr` is not yet stable as a const fn
     |
1105 |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
     |                                  ^^^^^^^^^^^^^^^^
     |
     |
     = help: const-stable functions can only call other const-stable functions

error: `ptr::read::copy_nonoverlapping` is not yet stable as a const fn
     |
1105 |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     |
     = help: const-stable functions can only call other const-stable functions

error: `maybe_uninit::MaybeUninit::<T>::as_mut_ptr` is not yet stable as a const fn
     |
     |
1198 |         copy_nonoverlapping(src as *const u8, tmp.as_mut_ptr() as *mut u8, mem::size_of::<T>());
     |
     = help: const-stable functions can only call other const-stable functions


error: `intrinsics::copy_nonoverlapping` is not yet stable as a const fn
     |
     |
1198 |         copy_nonoverlapping(src as *const u8, tmp.as_mut_ptr() as *mut u8, mem::size_of::<T>());
     |
     = help: const-stable functions can only call other const-stable functions

error: could not compile `core` due to 4 previous errors
