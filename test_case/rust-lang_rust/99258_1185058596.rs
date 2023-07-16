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
    Finished release [optimized] target(s) in 7.31s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: following path contains more than 2147 entries, you should move the test to some relevant subdirectory (current: 2149): /checkout/src/test/ui/issues
* highest error code: E0789
Found 506 error codes
Found 0 error(s) in error codes
Done!
