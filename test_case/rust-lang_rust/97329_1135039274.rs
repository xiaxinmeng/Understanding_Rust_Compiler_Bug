plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0711]: feature `const_intrinsic_copy` is declared unstable, but was previously declared stable
    --> library/core/src/ptr/const_ptr.rs:1200:5
     |
1200 |     #[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]

error[E0711]: feature `const_intrinsic_copy` is declared unstable, but was previously declared stable
    --> library/core/src/ptr/const_ptr.rs:1219:5
     |
     |
1219 |     #[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]

error[E0711]: feature `const_intrinsic_copy` is declared unstable, but was previously declared stable
    --> library/core/src/ptr/mut_ptr.rs:1312:5
     |
     |
1312 |     #[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]

error[E0711]: feature `const_intrinsic_copy` is declared unstable, but was previously declared stable
    --> library/core/src/ptr/mut_ptr.rs:1331:5
     |
     |
1331 |     #[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]

error[E0711]: feature `const_intrinsic_copy` is declared unstable, but was previously declared stable
    --> library/core/src/ptr/mut_ptr.rs:1350:5
     |
     |
1350 |     #[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]

error[E0711]: feature `const_intrinsic_copy` is declared unstable, but was previously declared stable
    --> library/core/src/ptr/mut_ptr.rs:1369:5
     |
     |
1369 |     #[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]

error[E0711]: feature `const_intrinsic_copy` is declared unstable, but was previously declared stable
    --> library/core/src/ptr/mod.rs:1092:9
     |
     |
1092 |         #[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]

error[E0711]: feature `const_intrinsic_copy` is declared unstable, but was previously declared stable
    --> library/core/src/ptr/mod.rs:1287:9
     |
     |
1287 |         #[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]

error: the feature `const_intrinsic_copy` has been stable since 1.63.0 and no longer requires an attribute to enable
   --> library/core/src/lib.rs:117:12
    |
    |
117 | #![feature(const_intrinsic_copy)]
    |            ^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D stable-features` implied by `-D warnings`
error: could not compile `core` due to 9 previous errors
Build completed unsuccessfully in 0:00:07
