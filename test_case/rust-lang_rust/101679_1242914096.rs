plain
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0425]: cannot find value `parent` in this scope
    --> compiler/rustc_middle/src/ty/sty.rs:1213:39
     |
1213 |         debug_assert_eq!(tcx.def_kind(parent), DefKind::Trait);

    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_middle` due to previous error
