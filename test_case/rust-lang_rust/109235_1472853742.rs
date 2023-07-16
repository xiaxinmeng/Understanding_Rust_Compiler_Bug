plain
    Checking addr2line v0.17.0
error[E0425]: cannot find value `e` in this scope
    --> library/std/src/sys/windows/fs.rs:1241:16
     |
1241 |             if e.raw_os_error() == Some(c::ERROR_CANNOT_ACCESS_FILE as u32) {

error[E0425]: cannot find value `ERROR_CANNOT_ACCESS_FILE` in module `c`
    --> library/std/src/sys/windows/fs.rs:1241:44
     |
     |
1241 |             if e.raw_os_error() == Some(c::ERROR_CANNOT_ACCESS_FILE as u32) {
     |                                            ^^^^^^^^^^^^^^^^^^^^^^^^ help: a constant with a similar name exists: `ERROR_CANT_ACCESS_FILE`
     |
    ::: library/std/src/sys/windows/c/errors.rs:719:1
     |
719  | pub const ERROR_CANT_ACCESS_FILE: DWORD = 1920;
     | ----------------------------------------------- similarly named constant `ERROR_CANT_ACCESS_FILE` defined here
error[E0615]: attempted to take value of method `file_type` on type `FileAttr`
    --> library/std/src/sys/windows/fs.rs:1247:22
     |
     |
1247 |             if attrs.file_type.is_symlink() {
     |
help: use parentheses to call the method
     |
     |
1247 |             if attrs.file_type().is_symlink() {

Some errors have detailed explanations: E0425, E0615.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `std` due to 3 previous errors
