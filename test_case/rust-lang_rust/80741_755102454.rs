plain
   Compiling rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
   Compiling rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error: use of deprecated associated function `std::path::Path::is_file`: other processes may remove, rename, or replace files at any time
    --> compiler/rustc_target/src/spec/mod.rs:1603:26
     |
1603 |                     if p.is_file() {
     |                          ^^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::File::open` or `std::fs::metadata``
     |
     = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated associated function `std::path::Path::is_file`: other processes may remove, rename, or replace files at any time
    --> compiler/rustc_target/src/spec/mod.rs:1610:32
     |
1610 |                 if target_path.is_file() {
     |                                ^^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::File::open` or `std::fs::metadata``

error: use of deprecated associated function `std::path::Path::canonicalize`: use `std::fs::canonicalize` instead
    --> compiler/rustc_target/src/spec/mod.rs:1799:39
1799 |         let canonicalized_path = path.canonicalize()?;
     |                                       ^^^^^^^^^^^^ help: replace the use of the deprecated associated function: `std::fs::canonicalize`

error: aborting due to 3 previous errors
