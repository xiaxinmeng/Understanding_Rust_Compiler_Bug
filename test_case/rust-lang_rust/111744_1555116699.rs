plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error: unused variable: `x_ptr`
   --> src/intrinsics/mod.rs:575:42
    |
575 |             intrinsic_args!(fx, args => (x_ptr, y_ptr); intrinsic);
    |                                          ^^^^^ help: if this is intentional, prefix it with an underscore: `_x_ptr`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `y_ptr`
   --> src/intrinsics/mod.rs:575:49
    |
    |
575 |             intrinsic_args!(fx, args => (x_ptr, y_ptr); intrinsic);
    |                                                 ^^^^^ help: if this is intentional, prefix it with an underscore: `_y_ptr`
error: could not compile `rustc_codegen_cranelift` (lib) due to 2 previous errors
Build completed unsuccessfully in 0:01:38
