plain
    Checking addr2line v0.16.0
error[E0308]: mismatched types
   --> library/std/src/sys/unix/process/process_common.rs:480:13
    |
480 |             debug_command.field("create_pidfd", &self.create_pidfd)
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: consider using a semicolon here: `;`
    |             |
    |             expected `()`, found `&mut DebugStruct<'_, '_>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:01:38
