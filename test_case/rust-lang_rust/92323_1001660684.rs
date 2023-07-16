plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused variable: `def_id`
   --> compiler/rustc_middle/src/mir/mono.rs:369:53
    |
369 | ...                   def.did.as_local().map(|def_id| tcx.def_span(def.did))
    |                                               ^^^^^^ help: if this is intentional, prefix it with an underscore: `_def_id`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:22
