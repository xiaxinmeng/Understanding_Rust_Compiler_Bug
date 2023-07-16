plain
    Checking object v0.26.2
    Checking hashbrown v0.12.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
error: couldn't read library/std/src/os/raw/../../../../core/src/ffi/c_void.md: No such file or directory (os error 2)
    --> library/std/src/os/raw/mod.rs:11:17
     |
8    | / macro_rules! alias_core_ffi {
9    | |     ($($t:ident)*) => {$(
10   | |         #[stable(feature = "raw_os", since = "1.1.0")]
11   | |         #[doc = include_str!(concat!("../../../../core/src/ffi/", stringify!($t), ".md"))]
...    |
19   | |     )*}
20   | | }
20   | | }
     | |_- in this expansion of `alias_core_ffi!` (#1)
21   | 
22   | / alias_core_ffi! {
23   | |     c_char c_schar c_uchar
24   | |     c_short c_ushort
25   | |     c_int c_uint
30   | |     c_void
31   | | }
     | |_- in this macro invocation (#1)
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1188:5
     |
1188 | /     macro_rules! include_str {
1189 | |         ($file:expr $(,)?) => {{ /* compiler built-in */ }};
     | |_____- in this expansion of `include_str!` (#2)

error[E0658]: use of unstable library feature 'raw_os_nonzero'
  --> library/std/src/sys/unix/process/process_unix.rs:10:5
---

error[E0658]: use of unstable library feature 'raw_os_nonzero'
   --> library/std/src/sys/unix/process/process_unix.rs:703:28
    |
703 | pub struct ExitStatusError(NonZero_c_int);
    |
    = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
    = help: add `#![feature(raw_os_nonzero)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'raw_os_nonzero'
   --> library/std/src/sys/unix/process/process_unix.rs:644:15
    |
644 |         match NonZero_c_int::try_from(self.0) {
    |
    = note: see issue #82363 <https://github.com/rust-lang/rust/issues/82363> for more information
    = help: add `#![feature(raw_os_nonzero)]` to the crate attributes to enable

