plain
---- src/ffi/c_str.rs - ffi::c_str::CString::from_vec_until_nul (line 838) stdout ----
error[E0433]: failed to resolve: use of undeclared type `CString`
   --> src/ffi/c_str.rs:851:13
    |
16  | let c_str = CString::from_vec_until_nul(buffer).unwrap();
    |
   ::: /checkout/library/alloc/src/string.rs:294:1
    |
294 | pub struct String {
294 | pub struct String {
    | ----------------- similarly named struct `String` defined here
    |
help: a struct with a similar name exists
    |
16  | let c_str = String::from_vec_until_nul(buffer).unwrap();
help: consider importing this struct
    |
5   | use std::ffi::CString;
    |
