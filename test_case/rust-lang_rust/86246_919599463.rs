plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: unused variable: `span`
   --> compiler/rustc_typeck/src/check/wfcheck.rs:197:22
    |
197 |     let (method_sig, span) = match trait_item.kind {
    |                      ^^^^ help: if this is intentional, prefix it with an underscore: `_span`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:03:13
