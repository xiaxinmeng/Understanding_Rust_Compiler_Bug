plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0308]: mismatched types
  --> src/abi.rs:52:46
   |
52 |                 option_kind.map(|kind| Reg { kind, size: self.prefix_chunk_size }.gcc_type(cx))
   |                                              ^^^^ expected enum `RegKind`, found struct `rustc_target::abi::call::Reg`

error[E0609]: no field `prefix_chunk_size` on type `&CastTarget`
   |
   |
52 |                 option_kind.map(|kind| Reg { kind, size: self.prefix_chunk_size }.gcc_type(cx))
   |
   |
   = note: available fields are: `prefix`, `rest`, `attrs`
Some errors have detailed explanations: E0308, E0609.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_gcc` due to 2 previous errors
Build completed unsuccessfully in 0:03:45
