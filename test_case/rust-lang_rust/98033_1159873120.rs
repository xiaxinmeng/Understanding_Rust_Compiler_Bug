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
  |
1 | use crate::io;
  |     ^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:18
