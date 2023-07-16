plain
   Compiling libc v0.2.88
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0711]: feature `cmp_min_max_by` is declared stable since 1.50.0, but was previously declared stable since 1.52.0
     |
     |
1091 | #[stable(feature = "cmp_min_max_by", since = "1.50.0")]


error[E0711]: feature `cmp_min_max_by` is declared stable since 1.50.0, but was previously declared stable since 1.52.0
     |
     |
1131 | #[stable(feature = "cmp_min_max_by", since = "1.50.0")]


error[E0711]: feature `cmp_min_max_by` is declared stable since 1.50.0, but was previously declared stable since 1.52.0
     |
     |
1153 | #[stable(feature = "cmp_min_max_by", since = "1.50.0")]

error: aborting due to 3 previous errors

error: could not compile `core`
