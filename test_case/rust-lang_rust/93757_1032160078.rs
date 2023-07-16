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
* 629 error codes
* highest error code: E0787
tidy error: /checkout/src/test/ui/generic-associated-types/bugs/issue-87748.rs:16: trailing whitespace
tidy error: /checkout/src/test/ui/generic-associated-types/bugs/issue-87803.rs:3: line longer than 100 chars
tidy error: /checkout/src/test/ui/generic-associated-types/bugs/issue-87803.rs:10: trailing whitespace
tidy error: /checkout/src/test/ui/generic-associated-types/bugs/issue-87803.rs:19: trailing whitespace
Found 0 error codes with no tests
Done!
* 363 features
some tidy checks failed
