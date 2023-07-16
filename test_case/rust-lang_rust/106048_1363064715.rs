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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 633 error codes
* highest error code: E0791
tidy error: /checkout/compiler/rustc_arena/src/tests.rs:6: TODO is deprecated; use FIXME
Found 0 error(s) in error codes
Done!
* 392 features
some tidy checks failed
