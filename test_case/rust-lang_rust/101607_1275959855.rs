plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking addr2line v0.16.0
error: unused import: `CStr`
 --> library/std/src/sys/windows/mod.rs:3:24
  |
3 | use crate::ffi::{cstr, CStr, OsStr, OsString};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:29
