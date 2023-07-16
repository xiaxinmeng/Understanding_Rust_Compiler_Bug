plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0308]: method not compatible with trait
   --> src/archive.rs:211:5
    |
211 | /     fn create_dll_import_lib(
212 | |         _sess: &'a Session,
213 | |         _lib_name: &str,
214 | |         _dll_imports: &[rustc_session::cstore::DllImport],
...   |
217 | |         bug!("creating dll imports is not supported");
    | |_____^ lifetime mismatch
    |
    |
    = note: expected fn pointer `fn(&Session, &str, &[DllImport], &std::path::Path) -> PathBuf`
               found fn pointer `fn(&'a Session, &str, &[DllImport], &std::path::Path) -> PathBuf`
note: the anonymous lifetime defined here...
    |
    |
213 |         _lib_name: &str,
    |                    ^^^^
note: ...does not necessarily outlive the lifetime `'a` as defined here
    |
    |
32  | impl<'a> ArchiveBuilder<'a> for ArArchiveBuilder<'a> {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:35
