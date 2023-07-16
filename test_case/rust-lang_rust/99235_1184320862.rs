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
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 630 error codes
tidy error: /checkout/src/librustdoc/clean/mod.rs:1946: trailing whitespace
Found 505 error codes
Found 0 error(s) in error codes
Done!
* 377 features
