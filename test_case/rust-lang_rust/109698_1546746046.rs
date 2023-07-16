plain
   Compiling addr2line v0.19.0
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   --> library/std/src/ffi/os_str.rs:703:26
    |
703 |         Self::from_inner(Slice::from_os_str_bytes_unchecked(bytes))
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
