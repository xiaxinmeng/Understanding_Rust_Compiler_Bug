plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: const-stable function cannot use `#[feature(const_fn_trait_bound)]`
   --> library/core/src/option.rs:530:6
    |
530 | impl<T> Option<T> {
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
1366|     #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
1366|     #[rustc_allow_const_fn_unstable(const_fn_trait_bound)]
    |

error: could not compile `core` due to previous error
