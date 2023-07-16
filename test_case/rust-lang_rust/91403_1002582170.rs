plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: unused variable: `fr_is_local`
   --> compiler/rustc_borrowck/src/diagnostics/region_errors.rs:543:13
    |
543 |             fr_is_local,
    |             ^^^^^^^^^^^ help: try ignoring the field: `fr_is_local: _`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_borrowck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:24
