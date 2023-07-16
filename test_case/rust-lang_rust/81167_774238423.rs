plain
   Compiling libc v0.2.85
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
    |
    |
896 |         copy_nonoverlapping(&src as *const T, dst, 1);


error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
    |
    |
992 |         copy_nonoverlapping(&src as *const T as *const u8, dst as *mut u8, mem::size_of::<T>());

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0492`.
