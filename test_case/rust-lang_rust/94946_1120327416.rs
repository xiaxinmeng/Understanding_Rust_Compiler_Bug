plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0711]: feature `ptr_metadata` is declared stable, but was previously declared unstable
   --> library/core/src/ptr/metadata.rs:108:1
    |
108 | #[rustc_const_stable(feature = "ptr_metadata", since = "1.61.0")]

error[E0711]: feature `const_slice_from_raw_parts` is declared unstable, but was previously declared stable
   --> library/core/src/ptr/mod.rs:714:1
    |
    |
714 | #[rustc_const_unstable(feature = "const_slice_from_raw_parts", issue = "67456")]

error[E0711]: feature `const_slice_from_raw_parts` is declared unstable, but was previously declared stable
   --> library/core/src/slice/raw.rs:132:1
    |
    |
132 | #[rustc_const_unstable(feature = "const_slice_from_raw_parts", issue = "67456")]

error: the feature `const_slice_from_raw_parts` has been stable since 1.61.0 and no longer requires an attribute to enable
   --> library/core/src/lib.rs:137:12
    |
    |
137 | #![feature(const_slice_from_raw_parts)]
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D stable-features` implied by `-D warnings`
error: could not compile `core` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:07
