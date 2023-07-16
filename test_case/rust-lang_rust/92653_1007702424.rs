plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.59 (/checkout/src/tools/clippy/clippy_lints)
error: hidden lifetime parameters in types are deprecated
    |
    |
137 |     result: Result<ConstValue<'tcx>, ErrorHandled>,
    |                                      ^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
    |
    = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:53
