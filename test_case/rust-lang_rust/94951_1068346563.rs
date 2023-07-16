plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0282]: type annotations needed
   --> compiler/rustc_mir_build/src/thir/pattern/check_match.rs:339:27
    |
339 |         let lint_affix = |affix, kind, suggestion| {
    |                           ^^^^^ consider giving this closure parameter a type
    = note: type must be known at this point

For more information about this error, try `rustc --explain E0282`.
error: could not compile `rustc_mir_build` due to previous error
