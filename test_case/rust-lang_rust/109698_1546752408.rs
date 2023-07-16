plain
   Compiling rand_xorshift v0.3.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling alloc v0.0.0 (/checkout/library/alloc)
error[E0599]: no function or associated item named `from_u8_slice` found for struct `sys::unix::os_str::Slice` in the current scope
  --> library/std/src/sys/unix/../unix/os_str/tests.rs:5:24
   |
5  |     let input = Slice::from_u8_slice(b"\xF0hello,\tworld");
   |
   |
  ::: library/std/src/sys/unix/os_str.rs:27:1
27 | pub struct Slice {
27 | pub struct Slice {
   | ---------------- function or associated item `from_u8_slice` not found for this struct
error[E0599]: no function or associated item named `from_u8_slice` found for struct `sys::unix::os_str::Slice` in the current scope
error[E0599]: no function or associated item named `from_u8_slice` found for struct `sys::unix::os_str::Slice` in the current scope
  --> library/std/src/sys/unix/../unix/os_str/tests.rs:16:16
   |
16 |         Slice::from_u8_slice(b"Hello\xC0\x80 There\xE6\x83 Goodbye").to_string(),
   |
   |
  ::: library/std/src/sys/unix/os_str.rs:27:1
27 | pub struct Slice {
27 | pub struct Slice {
   | ---------------- function or associated item `from_u8_slice` not found for this struct
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` (lib test) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:55
