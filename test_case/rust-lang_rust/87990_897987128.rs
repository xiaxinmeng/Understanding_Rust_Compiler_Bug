plain
    Checking cranelift-frontend v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking cranelift-native v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking cranelift-object v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0609]: no field `working_dir` on type `&Session`
  --> src/debuginfo/mod.rs:69:33
   |
69 |         let comp_dir = tcx.sess.working_dir.to_string_lossy(false).into_owned();
   |
   |
   = note: available fields are: `target`, `host`, `opts`, `host_tlib_path`, `target_tlib_path` ... and 19 others
help: one of the expressions' fields has a field of the same name
   |
69 |         let comp_dir = tcx.sess.opts.working_dir.to_string_lossy(false).into_owned();

For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:02:46
