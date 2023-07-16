plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.46
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: const-stable function cannot use `#[feature(const_mut_refs)]`
    |
    |
188 | pub const unsafe fn from_utf8_unchecked_mut(v: &mut [u8]) -> &mut str {
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
188 | #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
188 | #[rustc_allow_const_fn_unstable(const_mut_refs)]


error: const-stable function cannot use `#[feature(const_mut_refs)]`
    |
    |
191 |     unsafe { mem::transmute(v) }
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
188 | #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
188 | #[rustc_allow_const_fn_unstable(const_mut_refs)]


error: const-stable function cannot use `#[feature(const_mut_refs)]`
    |
    |
191 |     unsafe { mem::transmute(v) }
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
188 | #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
188 | #[rustc_allow_const_fn_unstable(const_mut_refs)]


error: const-stable function cannot use `#[feature(const_mut_refs)]`
    |
    |
191 |     unsafe { mem::transmute(v) }
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
188 | #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
188 | #[rustc_allow_const_fn_unstable(const_mut_refs)]


error: const-stable function cannot use `#[feature(const_mut_refs)]`
    |
    |
188 | pub const unsafe fn from_utf8_unchecked_mut(v: &mut [u8]) -> &mut str {
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
188 | #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
    |
188 | #[rustc_allow_const_fn_unstable(const_mut_refs)]

error: aborting due to 5 previous errors

error: could not compile `core`
