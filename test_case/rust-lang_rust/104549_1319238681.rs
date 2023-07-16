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
* 632 error codes
* highest error code: E0790
tidy error: /checkout/src/doc/unstable-book/src/compiler-flags/export-executable-symbols.md:12: trailing whitespace
tidy error: /checkout/src/doc/unstable-book/src/compiler-flags/export-executable-symbols.md:16: trailing whitespace
tidy error: /checkout/src/doc/unstable-book/src/compiler-flags/export-executable-symbols.md:28: trailing whitespace
tidy error: /checkout/src/doc/unstable-book/src/compiler-flags/export-executable-symbols.md:36: tab character
tidy error: /checkout/src/doc/unstable-book/src/compiler-flags/export-executable-symbols.md:37: tab character
Found 0 error(s) in error codes
Done!
* 390 features
some tidy checks failed
