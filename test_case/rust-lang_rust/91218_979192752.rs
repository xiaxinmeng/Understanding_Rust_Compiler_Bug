plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.52
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be `const`
    |
    |
726 |       #[rustc_const_unstable(feature = "str_ptr_get", issue = "74265")]
    |       ----------------------------------------------------------------- attribute specified here
727 |       #[inline]
728 | /     pub unsafe fn get_unchecked_mut<I>(self, index: I) -> NonNull<I::Output>
729 | |     where
730 | |         I: SliceIndex<str>,
    |
help: make the function or method const
   --> library/core/src/ptr/non_null.rs:728:5
    |
    |
728 | /     pub unsafe fn get_unchecked_mut<I>(self, index: I) -> NonNull<I::Output>
729 | |     where
730 | |         I: SliceIndex<str>,


error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be `const`
     |
     |
1107 |       #[rustc_const_unstable(feature = "str_ptr_get", issue = "74265")]
     |       ----------------------------------------------------------------- attribute specified here
1108 |       #[inline]
1109 | /     pub unsafe fn get_unchecked<I>(self, index: I) -> *const I::Output
1110 | |     where
1111 | |         I: SliceIndex<str>,
     |
help: make the function or method const
    --> library/core/src/ptr/const_ptr.rs:1109:5
     |
     |
1109 | /     pub unsafe fn get_unchecked<I>(self, index: I) -> *const I::Output
1110 | |     where
1111 | |         I: SliceIndex<str>,


error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be `const`
     |
     |
1425 |       #[rustc_const_unstable(feature = "str_ptr_get", issue = "74265")]
     |       ----------------------------------------------------------------- attribute specified here
1426 |       #[inline]
1427 | /     pub unsafe fn get_unchecked_mut<I>(self, index: I) -> *mut I::Output
1428 | |     where
1429 | |         I: SliceIndex<str>,
     |
help: make the function or method const
    --> library/core/src/ptr/mut_ptr.rs:1427:5
     |
     |
1427 | /     pub unsafe fn get_unchecked_mut<I>(self, index: I) -> *mut I::Output
1428 | |     where
1429 | |         I: SliceIndex<str>,

error: could not compile `core` due to 3 previous errors
Build completed unsuccessfully in 0:04:17
