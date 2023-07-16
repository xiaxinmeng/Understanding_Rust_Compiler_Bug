plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: mismatched types
  --> src/abi/pass_mode.rs:74:45
   |
74 |         .map(|&kind| reg_to_abi_param(Reg { kind, size: cast.prefix_chunk_size }))
   |                                             ^^^^ expected enum `RegKind`, found struct `rustc_target::abi::call::Reg`

error[E0609]: no field `prefix_chunk_size` on type `CastTarget`
  --> src/abi/pass_mode.rs:74:62
   |
74 |         .map(|&kind| reg_to_abi_param(Reg { kind, size: cast.prefix_chunk_size }))
   |
   |
   = note: available fields are: `prefix`, `rest`, `attrs`
Some errors have detailed explanations: E0308, E0609.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` due to 2 previous errors
Build completed unsuccessfully in 0:03:39
