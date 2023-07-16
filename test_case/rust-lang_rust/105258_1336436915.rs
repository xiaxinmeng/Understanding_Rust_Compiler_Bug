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
tidy error: /checkout/src/test/ui/impl-trait/in-trait/signature-mismatch.rs:15: line longer than 100 chars
tidy error: /checkout/src/test/ui/impl-trait/in-trait/signature-mismatch.rs:16: line longer than 100 chars
tidy error: /checkout/src/test/ui/impl-trait/in-trait/signature-mismatch.rs:32: line longer than 100 chars
tidy error: /checkout/src/test/ui/impl-trait/in-trait/signature-mismatch.rs:38: line longer than 100 chars
Found 0 error(s) in error codes
Done!
* 392 features
some tidy checks failed
