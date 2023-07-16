plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling regex v1.4.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 8.21s
tidy check
tidy error: following path contains more than 1458 entries, you should move the test to some relevant subdirectory (current: 1460): /checkout/src/test/ui
Found 435 error codes
Found 0 error codes with no tests
some tidy checks failed
Done!
