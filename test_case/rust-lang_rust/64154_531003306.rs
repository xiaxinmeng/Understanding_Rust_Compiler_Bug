
error[E0432]: unresolved import `crate::sys_common::backtrace`
  --> /home/travis/build/RalfJung/miri-test-libstd/rust-src-patched/src/libstd/backtrace.rs:98:24
   |
98 | use crate::sys_common::backtrace::{output_filename, lock};
   |                        ^^^^^^^^^ could not find `backtrace` in `sys_common`
