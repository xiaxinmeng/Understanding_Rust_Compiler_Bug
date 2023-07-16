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
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 7.62s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: following path contains more than 948 entries, you should move the test to some relevant subdirectory (current: 949): /checkout/src/test/ui
* 632 error codes
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
