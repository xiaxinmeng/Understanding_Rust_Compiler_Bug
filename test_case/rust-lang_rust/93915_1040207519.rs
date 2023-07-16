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
   Compiling regex v1.5.4
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 9.43s
tidy check
tidy error: following path contains more than 982 entries, you should move the test to some relevant subdirectory (current: 983): /checkout/src/test/ui
Checking which error codes lack tests...
* 629 error codes
* highest error code: E0787
Found 504 error codes
