plain
   |
13 | pub use crate::*;
   |         -------- the name `tests` in the type namespace is introduced by the glob reexport here
...
27 | mod tests;
   | ^^^^^^^^^^ but the local binding here shadows the name `tests` in the type namespace
   |
   = note: `-D local-binding-shadows-glob-reexport` implied by `-D warnings`
   Compiling rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
   Compiling rustc_abi v0.0.0 (/checkout/compiler/rustc_abi)
   Compiling rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
