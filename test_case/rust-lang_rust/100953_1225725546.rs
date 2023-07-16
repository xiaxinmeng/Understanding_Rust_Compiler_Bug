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
---- src/macros/mod.rs - macros::write (line 480) stdout ----
error: unused import: `Write as _`
 --> src/macros/mod.rs:481:22
  |
4 | use std::fmt::{self, Write as _};
  |
note: the lint level is defined here
 --> src/macros/mod.rs:478:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`

error: unused imports: `Write as _`, `self`
  |
  |
5 | use std::io::{self, Write as _};

error: aborting due to 2 previous errors

Couldn't compile the test.
