plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking addr2line v0.16.0
error[E0432]: unresolved import `crate::sys::locks`
 --> library/std/src/sys_common/condvar.rs:1:5
  |
1 | use crate::sys::locks as imp;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ no `locks` in `sys`
error[E0432]: unresolved import `crate::sys::locks`
 --> library/std/src/sys_common/condvar/check.rs:2:5
  |
2 | use crate::sys::locks as imp;
2 | use crate::sys::locks as imp;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ no `locks` in `sys`
error[E0432]: unresolved import `crate::sys::locks`
 --> library/std/src/sys_common/mutex.rs:1:5
  |
1 | use crate::sys::locks as imp;
1 | use crate::sys::locks as imp;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ no `locks` in `sys`
error[E0432]: unresolved import `crate::sys::locks`
 --> library/std/src/sys_common/remutex.rs:8:5
  |
  |
8 | use crate::sys::locks as sys;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ no `locks` in `sys`
error[E0432]: unresolved import `crate::sys::locks`
 --> library/std/src/sys_common/rwlock.rs:1:5
  |
1 | use crate::sys::locks as imp;
1 | use crate::sys::locks as imp;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ no `locks` in `sys`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `std` due to 5 previous errors
Build completed unsuccessfully in 0:00:16
