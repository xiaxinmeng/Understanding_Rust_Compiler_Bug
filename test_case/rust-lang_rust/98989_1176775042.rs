plain
    Checking cranelift-native v0.83.0
    Checking cranelift-frontend v0.83.0
    Checking cranelift-object v0.83.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0053]: method `create_dll_import_lib` has an incompatible type for trait
    |
    |
215 |         _tmpdir: &rustc_data_structures::temp_dir::MaybeTempDir,
    |                  |
    |                  expected struct `std::path::Path`, found struct `MaybeTempDir`
    |                  help: change the parameter type to match the trait: `&std::path::Path`
    |
    |
    = note: expected fn pointer `fn(&Session, &str, &[DllImport], &std::path::Path) -> PathBuf`
               found fn pointer `fn(&'a Session, &str, &[DllImport], &MaybeTempDir) -> PathBuf`
For more information about this error, try `rustc --explain E0053`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:02:54
