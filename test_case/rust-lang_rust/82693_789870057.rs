plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Checking which error codes lack tests...
Found 435 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs:148: trailing whitespace
tidy error: /checkout/compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs:149: unexplained "