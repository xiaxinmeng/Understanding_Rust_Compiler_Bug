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
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:93: trailing whitespace
tidy error: /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:108: trailing whitespace
tidy error: /checkout/src/test/ui-fulldeps/session-diagnostic/diagnostic-derive.rs:113: trailing whitespace
tidy error: /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:55: trailing whitespace
tidy error: /checkout/src/test/ui-fulldeps/session-diagnostic/subdiagnostic-derive.rs:406: trailing whitespace
some tidy checks failed
Build completed unsuccessfully in 0:00:11
