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
* 629 error codes
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
tidy error: /checkout/src/test/rustdoc/duplicate_impls/issue-33054.rs:5: line longer than 100 chars
* 366 features
some tidy checks failed
Build completed unsuccessfully in 0:00:11
