plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: unused variable: `def_scope`
     |
     |
1770 |                         let (adjusted_ident, def_scope) =
     |                                              ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_def_scope`
     |
     = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:43
