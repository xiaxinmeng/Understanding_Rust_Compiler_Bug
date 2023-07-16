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
    Finished release [optimized] target(s) in 10.01s
tidy check
tidy error: following path contains more than 1459 entries, you should move the test to some relevant subdirectory (current: 1461): /checkout/src/test/ui
Found 435 error codes
Found 0 error codes with no tests
Done!
some tidy checks failed
