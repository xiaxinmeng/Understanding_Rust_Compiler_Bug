plain
    Checking miniz_oxide v0.4.0
    Checking object v0.26.2
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
error[E0369]: binary operation `==` cannot be applied to type `io::error::Error`
    |
    |
691 |             r == 0 || crate::io::Error::last_os_error() == libc::EINTR,
    |                       --------------------------------- ^^ ----------- i32
    |                       io::error::Error
    |
    |
note: an implementation of `core::cmp::PartialEq<_>` might be missing for `io::error::Error`
    |
68  | pub struct Error {
68  | pub struct Error {
    | ^^^^^^^^^^^^^^^^ must implement `core::cmp::PartialEq<_>`
help: consider annotating `io::error::Error` with `#[derive(PartialEq)]`
   --> |library/std/src/io/error.rs:68:1
    |
68  | #[derive(PartialEq)]

For more information about this error, try `rustc --explain E0369`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:01:19
