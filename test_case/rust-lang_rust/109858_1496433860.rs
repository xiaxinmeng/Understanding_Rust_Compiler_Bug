plain
    Checking addr2line v0.17.0
error: unreachable call
  --> library/std/src/sys_common/thread_info.rs:25:42
   |
25 |         rtassert!(STACK_GUARD.with(|s| s.set(guard)).is_ok());
   |                                          ^^^ ----- any code following this expression is unreachable
   |                                          unreachable call
   |
   |
   = note: `-D unreachable-code` implied by `-D warnings`
error: unreachable pattern
  --> library/std/src/sys_common/thread_info.rs:24:12
   |
24 |     if let Some(guard) = stack_guard {
24 |     if let Some(guard) = stack_guard {
   |            ^^^^^^^^^^^
   |
   = note: `-D unreachable-patterns` implied by `-D warnings`
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:22
