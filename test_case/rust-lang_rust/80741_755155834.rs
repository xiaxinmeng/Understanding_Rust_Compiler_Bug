plain
   Compiling rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
   Compiling rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
  --> compiler/rustc_codegen_ssa/src/back/archive.rs:17:17
17 |         if test.exists() {
17 |         if test.exists() {
   |                 ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
   |
   = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
  --> compiler/rustc_codegen_ssa/src/back/archive.rs:22:21
22 |             if test.exists() {
22 |             if test.exists() {
   |                     ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
    --> compiler/rustc_codegen_ssa/src/back/link.rs:1113:18
     |
1113 |     if file_path.exists() {
     |                  ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
    --> compiler/rustc_codegen_ssa/src/back/link.rs:1119:22
     |
1119 |         if file_path.exists() {
     |                      ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
    --> compiler/rustc_codegen_ssa/src/back/link.rs:1125:22
     |
1125 |         if file_path.exists() {
     |                      ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::is_file`: other processes may remove, rename, or replace files at any time
    --> compiler/rustc_codegen_ssa/src/back/link.rs:1315:22
     |
1315 |         if full_path.is_file() && !full_path.starts_with(&sess.sysroot) {
     |                      ^^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::File::open` or `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
    --> compiler/rustc_codegen_ssa/src/back/link.rs:2231:64
     |
2231 |             _ if !p.is_absolute() || p == Path::new("/") || !p.exists() => {}
     |                                                                ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
error: aborting due to 7 previous errors

error: could not compile `rustc_codegen_ssa`

