plain
   Compiling tracing-tree v0.1.6
   Compiling chalk-solve v0.36.0
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
   --> compiler/rustc_session/src/filesearch.rs:157:65
    |
157 |             if sysroot.join(PRIMARY_LIB_DIR).join(RUST_LIB_DIR).exists() {
    |                                                                 ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
    |
    = note: `-D deprecated` implied by `-D warnings`
error: use of deprecated associated function `std::path::Path::symlink_metadata`: use `std::fs::symlink_metadata` instead
    --> compiler/rustc_session/src/session.rs:1387:41
     |
     |
1387 |         if let Ok(metadata) = candidate.symlink_metadata() {
     |                                         ^^^^^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::symlink_metadata`

error: use of deprecated associated function `std::path::Path::is_file`: other processes may remove, rename, or replace files at any time
     |
     |
1399 |         if candidate.join("library/std/src/lib.rs").is_file() { Some(candidate) } else { None }
     |                                                     ^^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::File::open` or `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
     |
     |
1480 |         if !path.exists() {
     |                  ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
error: use of deprecated associated function `std::path::Path::metadata`: use `std::fs::metadata` instead
  --> compiler/rustc_session/src/output.rs:42:13
   |
   |
42 |     match p.metadata() {
   |             ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::metadata`
error: aborting due to 5 previous errors

error: could not compile `rustc_session`

