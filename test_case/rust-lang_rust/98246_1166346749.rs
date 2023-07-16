plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking addr2line v0.16.0
error[E0308]: mismatched types
   --> library/std/src/sys/windows/fs.rs:562:35
    |
562 |         if times.accessed == Some(0) || times.modified == Some(0) {
    |                              ---- ^ expected struct `FILETIME`, found integer
    |                              arguments to this enum variant are incorrect


error[E0369]: binary operation `==` cannot be applied to type `core::option::Option<FILETIME>`
   --> library/std/src/sys/windows/fs.rs:562:27
    |
562 |         if times.accessed == Some(0) || times.modified == Some(0) {
    |            -------------- ^^ ------- core::option::Option<FILETIME>
    |            core::option::Option<FILETIME>
    |
    |
note: an implementation of `core::cmp::PartialEq` might be missing for `FILETIME`
   --> library/std/src/sys/windows/c.rs:611:1
611 | pub struct FILETIME {
611 | pub struct FILETIME {
    | ^^^^^^^^^^^^^^^^^^^ must implement `core::cmp::PartialEq`
help: consider annotating `FILETIME` with `#[derive(PartialEq)]`
    |
611 | #[derive(PartialEq)]

error[E0308]: mismatched types
   --> library/std/src/sys/windows/fs.rs:562:64
    |
    |
562 |         if times.accessed == Some(0) || times.modified == Some(0) {
    |                                                           ---- ^ expected struct `FILETIME`, found integer
    |                                                           arguments to this enum variant are incorrect


error[E0369]: binary operation `==` cannot be applied to type `core::option::Option<FILETIME>`
   --> library/std/src/sys/windows/fs.rs:562:56
    |
562 |         if times.accessed == Some(0) || times.modified == Some(0) {
    |                                         -------------- ^^ ------- core::option::Option<FILETIME>
    |                                         core::option::Option<FILETIME>
    |
    |
note: an implementation of `core::cmp::PartialEq` might be missing for `FILETIME`
   --> library/std/src/sys/windows/c.rs:611:1
611 | pub struct FILETIME {
611 | pub struct FILETIME {
    | ^^^^^^^^^^^^^^^^^^^ must implement `core::cmp::PartialEq`
help: consider annotating `FILETIME` with `#[derive(PartialEq)]`
    |
611 | #[derive(PartialEq)]

Some errors have detailed explanations: E0308, E0369.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `std` due to 4 previous errors
