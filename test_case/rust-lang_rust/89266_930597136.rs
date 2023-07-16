plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0432]: unresolved import `rustc_middle::middle::cstore`
 --> src/archive.rs:8:27
  |
8 | use rustc_middle::middle::cstore::DllImport;
  |                           ^^^^^^ could not find `cstore` in `middle`
error[E0432]: unresolved import `rustc_middle::middle::cstore`
  --> src/base.rs:10:27
   |
10 | use rustc_middle::middle::cstore::EncodedMetadata;
10 | use rustc_middle::middle::cstore::EncodedMetadata;
   |                           ^^^^^^ could not find `cstore` in `middle`
error[E0432]: unresolved import `rustc_middle::middle::cstore`
  --> src/lib.rs:64:27
   |
64 | use rustc_middle::middle::cstore::EncodedMetadata;
64 | use rustc_middle::middle::cstore::EncodedMetadata;
   |                           ^^^^^^ could not find `cstore` in `middle`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustc_codegen_gcc` due to 3 previous errors
Build completed unsuccessfully in 0:03:24
