plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
   --> compiler/rustc_codegen_llvm/src/back/lto.rs:500:25
    |
500 |                 if path.exists() { ThinLTOKeysMap::load_from_file(&path).ok() } else { None };
    |                         ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
    |
    = note: `-D deprecated` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_codegen_llvm`

