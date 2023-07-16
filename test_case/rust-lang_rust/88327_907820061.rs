plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: mismatched types
   --> src/abi/pass_mode.rs:108:54
    |
108 |                     let a = scalar_to_clif_type(tcx, a);
    |                                                      |
    |                                                      |
    |                                                      expected struct `rustc_target::abi::Scalar`, found `&rustc_target::abi::Scalar`
    |                                                      help: consider dereferencing the borrow: `*a`
error[E0308]: mismatched types
   --> src/abi/pass_mode.rs:109:54
    |
    |
109 |                     let b = scalar_to_clif_type(tcx, b);
    |                                                      |
    |                                                      |
    |                                                      expected struct `rustc_target::abi::Scalar`, found `&rustc_target::abi::Scalar`
    |                                                      help: consider dereferencing the borrow: `*b`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` due to 2 previous errors
Build completed unsuccessfully in 0:03:07
