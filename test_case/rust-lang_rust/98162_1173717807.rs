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
* 630 error codes
* highest error code: E0788
tidy error: /checkout/src/test/run-make-fulldeps/issue-84395-lto-embed-bitcode/test.rs:2: tab character
tidy error: /checkout/src/test/run-make-fulldeps/issue-84395-lto-embed-bitcode/test.rs: missing trailing newline
Found 0 error(s) in error codes
Done!
* 378 features
some tidy checks failed
