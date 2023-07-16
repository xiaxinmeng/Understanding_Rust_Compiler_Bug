plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0711]: feature `const_ptr_read` is declared unstable, but was previously declared stable
    --> library/core/src/ptr/mod.rs:1087:1
     |
1087 | #[rustc_const_unstable(feature = "const_ptr_read", issue = "80377")]

error[E0711]: feature `const_ptr_read` is declared unstable, but was previously declared stable
    --> library/core/src/ptr/mod.rs:1186:1
     |
     |
1186 | #[rustc_const_unstable(feature = "const_ptr_read", issue = "80377")]

error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:01:38
