plain
   Compiling addr2line v0.14.0
error: expected pattern, found `,`
   --> library/std/src/sys/unix/mod.rs:159:50
    |
159 |         libc::ENETDOWN => ErrorKind::NetworkDown,,
    |                                                  ^ expected pattern

error[E0599]: no variant or associated item named `NotSupported` found for enum `ErrorKind` in the current scope
   --> library/std/src/sys/unix/mod.rs:156:36
    |
156 |         libc::ENOSYS => ErrorKind::NotSupported,
    |                                    |
    |                                    variant or associated item not found in `ErrorKind`
    |                                    variant or associated item not found in `ErrorKind`
    |                                    help: there is a variant with a similar name: `Unsupported`
   ::: library/std/src/io/error.rs:95:1
    |
95  | pub enum ErrorKind {
95  | pub enum ErrorKind {
    | ------------------ variant or associated item `NotSupported` not found here
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `std`
