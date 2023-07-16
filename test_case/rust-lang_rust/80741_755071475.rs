plain
   Compiling crossbeam-queue v0.1.2
   Compiling psm v0.1.11
   Compiling stacker v0.1.12
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
    |
    |
208 |         } else if Path::new(lib).exists() {
    |                                  ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
    |
    = note: `-D deprecated` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_llvm`

