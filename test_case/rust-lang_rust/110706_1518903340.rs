plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0015]: cannot call non-const fn `transmute_unchecked::<[maybe_uninit::MaybeUninit<T>; N], [T; N]>` in constant functions
    |
950 |             intrinsics::transmute_unchecked(array)
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
