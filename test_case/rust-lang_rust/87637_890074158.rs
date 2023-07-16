plain
   Compiling rustc-demangle v0.1.18
error: const-stable function cannot use `#[feature(const_fn_trait_bound)]`
   --> library/alloc/src/vec/mod.rs:537:9
    |
537 | impl<T, A: Allocator> Vec<T, A> {
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
1820|     #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
1820|     #[rustc_allow_const_fn_unstable(const_fn_trait_bound)]
    |

error: const-stable function cannot use `#[feature(const_fn_trait_bound)]`
error: const-stable function cannot use `#[feature(const_fn_trait_bound)]`
   --> library/alloc/src/vec/mod.rs:537:9
    |
537 | impl<T, A: Allocator> Vec<T, A> {
    |
    |
help: if it is not part of the public API, make this function unstably const
    |
1837|     #[rustc_const_unstable(feature = "...", issue = "...")]
    |
help: otherwise `#[rustc_allow_const_fn_unstable]` can be used to bypass stability checks
1837|     #[rustc_allow_const_fn_unstable(const_fn_trait_bound)]
    |

error: aborting due to 2 previous errors
