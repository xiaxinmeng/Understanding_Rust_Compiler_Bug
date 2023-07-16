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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 629 error codes
* highest error code: E0787
tidy error: /checkout/src/etc/htmldocck.py:427: line longer than 100 chars
Found 0 error(s) in error codes
Done!
* 373 features
some tidy checks failed
