plain
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error: use of deprecated associated function `std::path::Path::canonicalize`: use `std::fs::canonicalize` instead
    |
    |
509 |     let input_path = input_path.canonicalize().ok();
    |                                 ^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::canonicalize`
    |
    = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated associated function `std::path::Path::canonicalize`: use `std::fs::canonicalize` instead
    |
    |
514 |         if output_path.canonicalize().ok() == input_path { Some(()) } else { None }
    |                        ^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::canonicalize`

error: use of deprecated associated function `std::path::Path::is_dir`: other processes may remove, rename, or replace directories at any time
    |
    |
520 |     let check = |output_path: &PathBuf| output_path.is_dir().then(|| output_path.clone());
    |                                                     ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
    |
    |
272 |         candidate.exists().then_some(candidate)
    |                   ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::canonicalize`: use `std::fs::canonicalize` instead
    |
    |
279 |     let path = current_dll_path().and_then(|s| s.canonicalize().ok());
    |                                                  ^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::canonicalize`

error: use of deprecated associated function `std::path::Path::exists`: other processes may remove or rename files at any time
    |
    |
400 |             f.exists()
    |               ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::read_dir`: use `std::fs::read_dir` instead
    |
    |
417 |     let d = sysroot.read_dir().unwrap_or_else(|e| {
    |                     ^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::read_dir`
error: aborting due to 7 previous errors

error: could not compile `rustc_interface`

