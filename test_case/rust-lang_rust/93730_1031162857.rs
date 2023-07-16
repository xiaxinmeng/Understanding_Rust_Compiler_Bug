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
Found 503 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/test/ui/parser/issue-93704-deeply-nested-expression.rs:3: line longer than 100 chars
tidy error: /checkout/src/test/ui/parser/issue-93704-deeply-nested-expression.rs: too many trailing newlines (2)
tidy error: /checkout/src/test/ui/parser/issue-79766-deeply-nested-expression.rs:5: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:00:12
