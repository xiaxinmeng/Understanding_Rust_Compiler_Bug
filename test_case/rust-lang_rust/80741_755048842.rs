plain
   Compiling termcolor v1.1.0
   Compiling annotate-snippets v0.8.0
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
   Compiling datafrog v2.0.1
error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
  --> compiler/rustc_fs_util/src/lib.rs:65:10
   |
65 |     if q.exists() {
   |          ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
   |
   = note: `-D deprecated` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_fs_util`

