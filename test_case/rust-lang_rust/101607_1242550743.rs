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
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/library/core/src/ffi/c_str.rs:336: `issue` "none" mismatches the previous `issue` of "95027"
tidy error: /checkout/library/std/src/error.rs:759: `issue` "95027" mismatches the previous `issue` of "none"
some tidy checks failed
