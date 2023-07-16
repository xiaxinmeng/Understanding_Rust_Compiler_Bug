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
2 | pub const INHERIT: u8 = 1;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D dead-code` implied by `-D warnings`
error: constant is never used: `SIGNAL`
  --> library/std/src/sys_common/../../../../compiler/rustc_session/src/config/sigpipe.rs:12:1
   |
12 | pub const SIGNAL: u8 = 3;
