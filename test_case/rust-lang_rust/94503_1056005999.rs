plain
    Checking addr2line v0.16.0
error: expected expression, found `$`
  --> library/std/src/os/raw/mod.rs:11:67
   |
8  | / macro_rules! alias_core_ffi {
9  | |     ($($t:ident)*) => {$(
10 | |         #[stable(feature = "raw_os", since = "1.1.0")]
11 | |         #[doc = include_str!(concat!("../../../../core/src/ffi/", $Docfile))]
...  |
19 | |     )*}
20 | | }
20 | | }
   | |_- in this expansion of `alias_core_ffi!`
21 | 
22 | / alias_core_ffi! {
23 | |     c_char c_schar c_uchar
24 | |     c_short c_ushort
25 | |     c_int c_uint
30 | |     c_void
31 | | }
   | |_- in this macro invocation


error[E0432]: unresolved import `crate::os::raw::NonZero_c_int`
 --> library/std/src/sys/unix/process/process_unix.rs:6:5
  |
6 | use crate::os::raw::NonZero_c_int;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `NonZero_c_int` in `os::raw`
error: unused import: `TryFrom`
 --> library/std/src/sys/unix/process/process_unix.rs:1:22
  |
  |
1 | use crate::convert::{TryFrom, TryInto};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error[E0658]: use of unstable library feature 'core_ffi_c'
  --> library/std/src/os/raw/mod.rs:18:23
   |
   |
8  | / macro_rules! alias_core_ffi {
9  | |     ($($t:ident)*) => {$(
10 | |         #[stable(feature = "raw_os", since = "1.1.0")]
11 | |         #[doc = include_str!(concat!("../../../../core/src/ffi/", $Docfile))]
...  |
18 | |         pub type $t = core::ffi::$t;
19 | |     )*}
20 | | }
20 | | }
   | |_- in this expansion of `alias_core_ffi!`
21 | 
22 | / alias_core_ffi! {
23 | |     c_char c_schar c_uchar
24 | |     c_short c_ushort
25 | |     c_int c_uint
30 | |     c_void
31 | | }
   | |_- in this macro invocation
   |
   |
   = note: see issue #94501 <https://github.com/rust-lang/rust/issues/94501> for more information
   = help: add `#![feature(core_ffi_c)]` to the crate attributes to enable
Some errors have detailed explanations: E0432, E0658.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `std` due to 4 previous errors
Build completed unsuccessfully in 0:01:35
