plain
..........................................................iiiiii........................ 1144/1199
.......................i...............................
failures:

---- src/ffi/os_str.rs - ffi::os_str::OsString (line 53) stdout ----
error[E0412]: cannot find type `OsStr` in this scope
  |
  |
4 | fn concat_os_strings(a: &OsStr, b: &OsStr) -> OsString {
  |
help: consider importing this struct
  |
3 | use std::ffi::OsStr;
3 | use std::ffi::OsStr;
  |

error[E0412]: cannot find type `OsStr` in this scope
  |
  |
4 | fn concat_os_strings(a: &OsStr, b: &OsStr) -> OsString {
  |
help: consider importing this struct
  |
3 | use std::ffi::OsStr;
3 | use std::ffi::OsStr;
  |

error[E0412]: cannot find type `OsString` in this scope
    |
    |
4   | fn concat_os_strings(a: &OsStr, b: &OsStr) -> OsString {
    |
   ::: /checkout/library/alloc/src/string.rs:366:1
    |
366 | pub struct String {
366 | pub struct String {
    | ----------------- similarly named struct `String` defined here
    |
help: a struct with a similar name exists
    |
4   | fn concat_os_strings(a: &OsStr, b: &OsStr) -> String {
help: consider importing this struct
    |
3   | use std::ffi::OsString;
    |
    |

error[E0433]: failed to resolve: use of undeclared type `OsString`
   --> src/ffi/os_str.rs:55:15
    |
5   |     let ret = OsString::with_capacity(a.len() + b.len()); // This will allocate
    |
   ::: /checkout/library/alloc/src/string.rs:366:1
    |
366 | pub struct String {
366 | pub struct String {
    | ----------------- similarly named struct `String` defined here
    |
help: a struct with a similar name exists
    |
5   |     let ret = String::with_capacity(a.len() + b.len()); // This will allocate
help: consider importing this struct
    |
3   | use std::ffi::OsString;
    |
