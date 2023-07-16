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
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unused import: `PathBuf`
 --> src/librustdoc/errors.rs:6:23
  |
6 | use std::path::{Path, PathBuf};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:01:41
