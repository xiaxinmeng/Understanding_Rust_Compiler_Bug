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
tidy error: following path contains more than 2147 entries, you should move the test to some relevant subdirectory (current: 2148): /checkout/src/test/ui/issues
Found 0 error(s) in error codes
Done!
* 380 features
some tidy checks failed
