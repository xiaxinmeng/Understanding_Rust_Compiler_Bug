plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.69
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: const-stable function cannot use `#[feature(const_mut_refs)]`
    |
    |
483 |     pub const fn as_mut_ptr(&mut self) -> *mut T {
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
483 |     #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
483 |     #[rustc_allow_const_fn_unstable(const_mut_refs)]


error: const-stable function cannot use `#[feature(const_mut_refs)]`
    |
    |
560 |     pub const fn as_mut_ptr_range(&mut self) -> Range<*mut T> {
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
560 |     #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
560 |     #[rustc_allow_const_fn_unstable(const_mut_refs)]


error: const-stable function cannot use `#[feature(const_mut_refs)]`
    |
561 |         let start = self.as_mut_ptr();
    |                     ^^^^^^^^^^^^^^^^^
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
560 |     #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
560 |     #[rustc_allow_const_fn_unstable(const_mut_refs)]

error: could not compile `core` due to 3 previous errors
Build completed unsuccessfully in 0:00:10
