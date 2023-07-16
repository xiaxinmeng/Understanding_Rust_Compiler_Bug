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
* 630 error codes
* highest error code: E0788
tidy error: /checkout/src/doc/rustc/src/platform-support/armv4t_none_eabi.md:14: trailing whitespace
Found 0 error(s) in error codes
Done!
* 379 features
some tidy checks failed
