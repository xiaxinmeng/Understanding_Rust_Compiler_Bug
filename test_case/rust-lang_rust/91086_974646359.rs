plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.52
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: an `#[unstable]` annotation here has no effect
    |
    |
179 | #[unstable(feature = "from_mut_slice_to_array", issue = "91085")]
    |
    |
    = note: `#[deny(ineffective_unstable_trait_impl)]` on by default

error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:01:10
