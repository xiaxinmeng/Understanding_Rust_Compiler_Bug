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
Found 504 error codes
Found 0 error(s) in error codes
Done!
* 375 features
tidy error: /checkout/library/std/src/error.rs:45: TODO is deprecated; use FIXME
some tidy checks failed
