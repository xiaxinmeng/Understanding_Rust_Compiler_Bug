plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0046]: not all trait items implemented, missing: `unpack_archive`
  --> src/archive.rs:57:1
   |
57 | impl<'a> ArchiveBuilder<'a> for ArArchiveBuilder<'a> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `unpack_archive` in implementation
   |
   = help: implement the missing item: `fn unpack_archive(self: std::boxed::Box<Self>, _: &PathBuf, _: &std::path::Path, _: &mut (dyn for<'r> std::ops::FnMut(&'r str) -> bool + 'static)) -> std::result::Result<(), std::io::Error> { todo!() }`
For more information about this error, try `rustc --explain E0046`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:02:57
