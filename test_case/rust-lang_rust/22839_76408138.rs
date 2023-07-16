

---- [run-pass] run-pass/backtrace-debuginfo.rs stdout ----

error: test run failed!
status: exit code: 101
command: i686-apple-darwin/test/run-pass/backtrace-debuginfo.stage2-i686-apple-darwin 
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<main>' panicked at 'trace does not match position list: test case 0
thread '<main>' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/run-pass/backtrace-debuginfo.rs:51
stack backtrace:
   1:   0x1927fa - sys::backtrace::write::hb8abcf3ad25ae5f8lmA
   2:   0x1bd313 - panicking::on_panic::h436869a0a896a62fw8I
   3:    0xedef4 - rt::unwind::begin_unwind_inner::h3a980d3b167b71basRI
   4:    0xd9470 - rt::unwind::begin_unwind::h14711308078462879468
   5:    0xd931a - inner::h1efe2fbb2d14b47csca
   6:    0xd970a - outer::hb4f86a2ea3846118rla
   7:    0xdf337 - main::h8d1983e9f237c03d4qa
   8:   0x22c36d - rust_try_inner
   9:   0x22c346 - rust_try
  10:   0x1bddc4 - rt::lang_start::h0ebea00bcd9cc770c3I
  11:    0xdf521 - main

---
backtrace-debuginfo.rs:86
backtrace-debuginfo.rs:133
', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/run-pass/backtrace-debuginfo.rs:100

------------------------------------------

thread '[run-pass] run-pass/backtrace-debuginfo.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/compiletest/runtest.rs:1466

