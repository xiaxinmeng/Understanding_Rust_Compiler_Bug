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
tidy error: /checkout/src/test/rustdoc-json/nested.rs: too many trailing newlines (2)
tidy error: /checkout/src/test/rustdoc-json/reexport/glob_private.rs: missing trailing newline
tidy error: /checkout/src/test/rustdoc-json/reexport/auxiliary/pub-struct.rs: missing trailing newline
tidy error: /checkout/src/test/rustdoc-json/reexport/private_twice_one_inline.rs: missing trailing newline
tidy error: /checkout/src/test/rustdoc-json/reexport/simple_private.rs: missing trailing newline
Found 0 error(s) in error codes
Done!
* 379 features
some tidy checks failed
