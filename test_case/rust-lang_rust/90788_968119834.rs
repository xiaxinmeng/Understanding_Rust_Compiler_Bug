plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: unused variable: `statement`
   --> compiler/rustc_mir_dataflow/src/impls/mod.rs:449:9
    |
449 |         statement: &mir::Statement<'tcx>,
    |         ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_statement`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `terminator`
   --> compiler/rustc_mir_dataflow/src/impls/mod.rs:463:9
    |
    |
463 |         terminator: &mir::Terminator<'tcx>,
    |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_terminator`
error: could not compile `rustc_mir_dataflow` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:17
