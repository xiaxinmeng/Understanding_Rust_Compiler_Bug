
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   --> library/std/src/sys/unix/thread.rs:342:48
    |
342 |             let mut sinfo: libc::system_info = crate::mem::zeroed();
    |                                                ^^^^^^^^^^^^^^^^^^^^ call to unsafe function
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   --> library/std/src/sys/unix/thread.rs:343:23
    |
343 |             let res = libc::get_system_info(&mut sinfo);
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

