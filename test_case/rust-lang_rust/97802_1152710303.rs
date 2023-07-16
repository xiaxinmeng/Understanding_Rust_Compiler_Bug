plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: mismatched types
   --> src/main_shim.rs:147:78
    |
147 |                     bcx.ins().call(func_ref, &[main_val, arg_argc, arg_argv, ignore_sigpipe]);
    |                                                                              ^^^^^^^^^^^^^^ expected struct `cranelift_codegen::ir::Value`, found `bool`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:46
