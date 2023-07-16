plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0711]: feature `strict_provenance` is declared stable, but was previously declared unstable
    |
    |
513 | #[rustc_const_stable(feature = "strict_provenance", since = "1.61.0")]


error[E0711]: feature `strict_provenance` is declared stable, but was previously declared unstable
    |
    |
540 | #[rustc_const_stable(feature = "strict_provenance", since = "1.61.0")]

error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:03:50
