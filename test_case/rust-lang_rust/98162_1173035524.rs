plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0609]: no field `emit_thin_lto` on type `CodegenOptions`
    |
    |
577 |     tracked!(emit_thin_lto, false);
    |
    |
    = note: available fields are: `ar`, `code_model`, `codegen_units`, `control_flow_guard`, `debug_assertions` ... and 41 others
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_interface` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:02:59
