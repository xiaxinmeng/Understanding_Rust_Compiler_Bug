plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0308]: mismatched types
   --> compiler/rustc_codegen_llvm/src/asm.rs:145:67
    |
145 | ...                   let feature_name = Symbol::intern(feature);
    |                                                         ^^^^^^^ expected `&str`, found struct `rustc_span::Symbol`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_llvm` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
