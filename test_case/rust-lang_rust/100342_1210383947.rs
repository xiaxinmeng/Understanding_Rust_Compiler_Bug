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
---- src/mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit (line 34) stdout ----
error: unknown lint: `mem_uninitialized`
 --> src/mem/maybe_uninit.rs:34:25
  |
3 | #![allow(invalid_value, mem_uninitialized)]
  |
note: the lint level is defined here
 --> src/mem/maybe_uninit.rs:32:9
  |
---
---- src/mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit (line 49) stdout ----
error: unknown lint: `mem_uninitialized`
 --> src/mem/maybe_uninit.rs:49:25
  |
3 | #![allow(invalid_value, mem_uninitialized)]
  |
note: the lint level is defined here
 --> src/mem/maybe_uninit.rs:47:9
  |
