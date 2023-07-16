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
* highest error code: E0791
Found 507 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/src/doc/unstable-book/src/compiler-flags/sanitizer.md:427: unexplained "