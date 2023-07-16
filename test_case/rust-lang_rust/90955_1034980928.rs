plain
   Compiling object v0.26.2
   Compiling hashbrown v0.12.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
error[E0599]: no variant or associated item named `FilenameTooLong` found for enum `ErrorKind` in the current scope
    |
318 |         FilenameTooLong,
    |         ^^^^^^^^^^^^^^^ variant or associated item not found in `ErrorKind`
    |
    |
   ::: library/std/src/io/error.rs:148:1
    |
148 | pub enum ErrorKind {
    | ------------------ variant or associated item `FilenameTooLong` not found here
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:21
