plain
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.12.3
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error[E0271]: expected `F` to be a type parameter that returns `core::result::Result<_, io::error::Error>`, but it returns `T`
   |
   |
8  | pub fn with_native_path<T, F: FnOnce(&NativePath) -> T>(path: &Path, f: F) -> T {
   |                         - this type parameter
9  |     run_path_with_cstr(path, f)
   |     ------------------       ^ expected enum `core::result::Result`, found type parameter `T`
   |     required by a bound introduced by this call
   |
   = note:        expected enum `core::result::Result<_, io::error::Error>`
           found type parameter `T`
           found type parameter `T`
note: required by a bound in `run_path_with_cstr`
  --> library/std/src/sys/common/small_c_string.rs:20:25
   |
18 | pub fn run_path_with_cstr<T, F>(path: &Path, f: F) -> io::Result<T>
19 | where
19 | where
20 |     F: FnOnce(&CStr) -> io::Result<T>,
   |                         ^^^^^^^^^^^^^ required by this bound in `run_path_with_cstr`
error[E0308]: mismatched types
 --> library/std/src/sys/unix/path.rs:9:5
  |
  |
8 | pub fn with_native_path<T, F: FnOnce(&NativePath) -> T>(path: &Path, f: F) -> T {
  |                         - this type parameter                                 - expected `T` because of return type
9 |     run_path_with_cstr(path, f)
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found enum `core::result::Result`
  = note: expected type parameter `T`
                       found enum `core::result::Result<_, io::error::Error>`

Some errors have detailed explanations: E0271, E0308.
