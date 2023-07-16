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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 626 error codes
* highest error code: E0784
tidy error: /checkout/src/test/ui/generic-associated-types/issue-87429-2.rs: missing trailing newline
tidy error: /checkout/compiler/rustc_typeck/src/check/compare_method.rs:1249: unexplained "