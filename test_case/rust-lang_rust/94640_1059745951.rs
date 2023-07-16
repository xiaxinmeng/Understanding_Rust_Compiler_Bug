plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0711]: feature `slice_ptr_len` is declared unstable, but was previously declared stable
    |
    |
989 |     #[unstable(feature = "slice_ptr_len", issue = "71146")]


error[E0711]: feature `slice_ptr_len` is declared unstable, but was previously declared stable
     |
     |
1259 |     #[unstable(feature = "slice_ptr_len", issue = "71146")]

error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:00:09
