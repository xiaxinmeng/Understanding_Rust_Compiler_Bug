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
error[E0432]: unresolved import `crate::char::MAX_UTF8`
 --> library/std/src/sys/windows/stdio.rs:3:33
  |
3 | use crate::char::{decode_utf16, MAX_UTF8};
  |                                 ^^^^^^^^ no `MAX_UTF8` in `char`

error[E0425]: cannot find value `MAX_UTF8_LEN` in this scope
   |
   |
30 |     bytes: [u8; MAX_UTF8_LEN],
   |
help: consider importing one of these items
   |
3  | use core::char::MAX_UTF8_LEN;
3  | use core::char::MAX_UTF8_LEN;
   |
3  | use crate::char::MAX_UTF8_LEN;
   |

error[E0425]: cannot find value `MAX_UTF8_LEN` in this scope
    |
    |
380 |         IncompleteUtf8 { bytes: [0; MAX_UTF8_LEN], len: 0 }
    |
help: consider importing one of these items
    |
3   | use core::char::MAX_UTF8_LEN;
