plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0015]: cannot call non-const fn `ParamEnv::<'_>::new` in constant functions
     |
     |
1665 |         Self::new(List::empty(), Reveal::UserFacing, hir::Constness::NotConst)
     |
     |
     = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

error[E0015]: cannot call non-const fn `ParamEnv::<'_>::new` in constant functions
     |
     |
1697 |         Self::new(List::empty(), Reveal::All, hir::Constness::NotConst)
     |
     |
     = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
For more information about this error, try `rustc --explain E0015`.
error: could not compile `rustc_middle` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to 2 previous errors
