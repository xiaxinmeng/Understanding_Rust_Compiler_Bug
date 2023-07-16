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
tidy error: /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_unfulfilled_expectation.rs:17: line longer than 100 chars
tidy error: /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_unfulfilled_expectation.rs:22: trailing whitespace
tidy error: /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_unfulfilled_expectation.rs:24: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:00:12
