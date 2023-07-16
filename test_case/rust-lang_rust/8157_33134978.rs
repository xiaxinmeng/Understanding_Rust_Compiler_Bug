
src/librustdoc/plugins.rs:    priv dylibs: ~[dl::DynamicLibrary],
src/librustdoc/plugins.rs:        let lib_result = dl::DynamicLibrary::open(Some(&x));
src/libstd/unstable/dynamic_lib.rs:pub struct DynamicLibrary { priv handle: *libc::c_void }
src/libstd/unstable/dynamic_lib.rs:impl Drop for DynamicLibrary {
src/libstd/unstable/dynamic_lib.rs:impl DynamicLibrary {
src/libstd/unstable/dynamic_lib.rs:    pub fn open(filename: Option<&path::Path>) -> Result<DynamicLibrary, ~str> {
src/libstd/unstable/dynamic_lib.rs:                Ok(handle) => Ok(DynamicLibrary { handle: handle })
src/libstd/unstable/dynamic_lib.rs:        let libm = match DynamicLibrary::open(None) {
src/libstd/unstable/dynamic_lib.rs:        match DynamicLibrary::open(Some(&path)) {
src/libsyntax/ext/expand.rs:use std::unstable::dynamic_lib::DynamicLibrary;
src/libsyntax/ext/expand.rs:    let lib = match DynamicLibrary::open(Some(&path)) {
src/libsyntax/ext/base.rs:use std::unstable::dynamic_lib::DynamicLibrary;
src/libsyntax/ext/base.rs:    macro_crates: ~[DynamicLibrary],
src/libsyntax/ext/base.rs:    pub fn insert_macro_crate(&mut self, lib: DynamicLibrary) {
