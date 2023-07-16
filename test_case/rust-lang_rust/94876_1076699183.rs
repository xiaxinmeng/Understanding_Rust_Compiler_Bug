plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: unused import: `expr::as_constant`
    --> compiler/rustc_mir_build/src/build/mod.rs:1081:11
     |
1081 | crate use expr::as_constant;
     |
     |
     = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: could not compile `rustc_mir_build` due to previous error
warning: build failed, waiting for other jobs to finish...
