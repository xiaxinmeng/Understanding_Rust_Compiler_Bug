
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'assertion failed: slice_layout_is_correct(cx, &member_llvm_types[..], element_type)', ../src/librustc_trans\trans\debuginfo\metadata.rs:541
stack backtrace:
   0:         0x6ff00e07 - strncmp
   1:         0x6ff08538 - strncmp
   2:         0x6fec537c - strncmp
   3:         0x6c1cb7f3 - strncmp
   4:         0x6c2cd6a1 - strncmp
   5:         0x6c2cb316 - strncmp
   6:         0x6c22274e - strncmp
   7:         0x6c234e1b - strncmp
   8:         0x6c238cac - strncmp
   9:         0x6c23cc6c - strncmp
