plain

error[E0433]: failed to resolve: use of undeclared crate or module `io`
  --> library/std/src/sys/sgx/path.rs:22:41
   |
22 | pub(crate) fn absolute(_path: &Path) -> io::Result<PathBuf> {
   |                                         ^^ use of undeclared crate or module `io`
For more information about this error, try `rustc --explain E0433`.
[RUSTC-TIMING] std test:false 1.910
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
