plain
   Compiling rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
   Compiling rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
   Compiling rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
  --> compiler/rustc_incremental/src/persist/file_format.rs:55:14
   |
55 |     if !path.exists() {
   |              ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
   |
   = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated associated function `std::path::Path::canonicalize`: use `std::fs::canonicalize` instead
   --> compiler/rustc_incremental/src/persist/fs.rs:208:37
    |
208 |     let crate_dir = match crate_dir.canonicalize() {
    |                                     ^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::canonicalize`

error: use of deprecated associated function `std::path::Path::read_dir`: use `std::fs::read_dir` instead
   --> compiler/rustc_incremental/src/persist/fs.rs:390:58
    |
390 |     let sess_dir_iterator = sess.incr_comp_session_dir().read_dir()?;
    |                                                          ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::read_dir`

error: use of deprecated associated function `std::path::Path::read_dir`: use `std::fs::read_dir` instead
   --> compiler/rustc_incremental/src/persist/fs.rs:415:48
    |
415 |     let source_dir_iterator = match source_dir.read_dir() {
    |                                                ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::read_dir`

error: use of deprecated associated function `std::path::Path::read_dir`: use `std::fs::read_dir` instead
   --> compiler/rustc_incremental/src/persist/fs.rs:538:10
    |
538 |         .read_dir()
    |          ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::read_dir`

error: use of deprecated associated function `std::path::Path::read_dir`: use `std::fs::read_dir` instead
   --> compiler/rustc_incremental/src/persist/fs.rs:673:38
    |
673 |     for dir_entry in crate_directory.read_dir()? {
    |                                      ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::read_dir`

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
   --> compiler/rustc_incremental/src/persist/fs.rs:925:10
    |
925 |     if p.exists() {
    |          ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::canonicalize`: use `std::fs::canonicalize` instead
   --> compiler/rustc_incremental/src/persist/fs.rs:926:31
926 |         let canonicalized = p.canonicalize()?;
    |                               ^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::canonicalize`


error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
   --> compiler/rustc_incremental/src/persist/fs.rs:934:10
    |
934 |     if p.exists() {
    |          ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::canonicalize`: use `std::fs::canonicalize` instead
   --> compiler/rustc_incremental/src/persist/fs.rs:935:31
935 |         let canonicalized = p.canonicalize()?;
    |                               ^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::canonicalize`


error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
   --> compiler/rustc_incremental/src/persist/load.rs:145:30
    |
145 |                     if !path.exists() {
    |                              ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
  --> compiler/rustc_incremental/src/persist/save.rs:78:62
   |
78 |                     !in_incr_comp_dir_sess(sess, &file_name).exists()
   |                                                              ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
  --> compiler/rustc_incremental/src/persist/save.rs:90:30
   |
90 |             .all(|path| path.exists())
   |                              ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
   --> compiler/rustc_incremental/src/persist/save.rs:104:17
    |
104 |     if path_buf.exists() {
    |                 ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
error: aborting due to 14 previous errors

error: could not compile `rustc_incremental`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: use of deprecated associated function `std::path::Path::canonicalize`: use `std::fs::canonicalize` instead
   --> compiler/rustc_metadata/src/locator.rs:594:39
    |
594 |                 let sysroot = sysroot.canonicalize().unwrap_or_else(|_| sysroot.to_path_buf());
    |                                       ^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::canonicalize`
    |
    = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
   --> compiler/rustc_metadata/src/locator.rs:667:21
    |
667 |             if !loc.exists() {
    |                     ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
   --> compiler/rustc_metadata/src/locator.rs:741:18
    |
741 |     if !filename.exists() {
    |                  ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``
error: aborting due to 3 previous errors

error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
