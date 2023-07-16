plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: const-stable function cannot use `#[feature(const_mut_refs)]`
     |
1104 |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
     |                                  ^^^^^^^^^^^^^^^^
     |
     |
help: if it is not part of the public API, make this function unstably const
     |
1088 | #[rustc_const_unstable(feature = "...", issue = "...")]
     |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
     |
1088 | #[rustc_allow_const_fn_unstable(const_mut_refs)]


error: `maybe_uninit::MaybeUninit::<T>::as_mut_ptr` is not yet stable as a const fn
     |
1104 |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
     |                                  ^^^^^^^^^^^^^^^^
     |
     |
     = help: const-stable functions can only call other const-stable functions

error: `ptr::read::copy_nonoverlapping` is not yet stable as a const fn
     |
1104 |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     |
     = help: const-stable functions can only call other const-stable functions

error: const-stable function cannot use `#[feature(const_mut_refs)]`
     |
     |
1196 |         copy_nonoverlapping(src as *const u8, tmp.as_mut_ptr() as *mut u8, mem::size_of::<T>());
     |
     |
help: if it is not part of the public API, make this function unstably const
     |
1187 | #[rustc_const_unstable(feature = "...", issue = "...")]
     |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
     |
1187 | #[rustc_allow_const_fn_unstable(const_mut_refs)]


error: `maybe_uninit::MaybeUninit::<T>::as_mut_ptr` is not yet stable as a const fn
     |
     |
1196 |         copy_nonoverlapping(src as *const u8, tmp.as_mut_ptr() as *mut u8, mem::size_of::<T>());
     |
     = help: const-stable functions can only call other const-stable functions


error: `intrinsics::copy_nonoverlapping` is not yet stable as a const fn
     |
     |
1196 |         copy_nonoverlapping(src as *const u8, tmp.as_mut_ptr() as *mut u8, mem::size_of::<T>());
     |
     = help: const-stable functions can only call other const-stable functions

error: could not compile `core` due to 6 previous errors
