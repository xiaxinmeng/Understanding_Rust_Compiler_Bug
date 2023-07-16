plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0412]: cannot find type `DllImport` in this scope
   --> src/archive.rs:214:25
    |
214 |         _dll_imports: &[DllImport],
    |
help: consider importing this struct
    |
3   | use rustc_session::cstore::DllImport;
3   | use rustc_session::cstore::DllImport;
    |

error[E0412]: cannot find type `MaybeTempDir` in this scope
   --> src/archive.rs:215:19
    |
215 |         _tmpdir: &MaybeTempDir,
    |
help: consider importing this struct
    |
3   | use rustc_data_structures::temp_dir::MaybeTempDir;
