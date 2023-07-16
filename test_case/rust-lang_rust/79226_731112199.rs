
error: Undefined Behavior: scalar size mismatch: expected 8 bytes but got 4 bytes instead
   --> /checkout/library/std/src/sys/unix/fs.rs:107:9
    |
107 | /         syscall! {
108 | |             fn statx(
109 | |                 fd: c_int,
110 | |                 pathname: *const libc::c_char,
...   |
114 | |             ) -> c_int
115 | |         }
    | |_________^ scalar size mismatch: expected 8 bytes but got 4 bytes instead
