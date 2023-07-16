plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0711]: feature `ptr_metadata` is declared stable, but was previously declared unstable
   --> library/core/src/ptr/metadata.rs:108:1
    |
108 | #[rustc_const_stable(feature = "ptr_metadata", since = "1.62.0")]

error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:01:25
