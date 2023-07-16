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
* 631 error codes
* highest error code: E0789
tidy error: /checkout/src/librustdoc/html/static/css/rustdoc.css:1019: line longer than 100 chars
Found 0 error(s) in error codes
Done!
* 379 features
some tidy checks failed
