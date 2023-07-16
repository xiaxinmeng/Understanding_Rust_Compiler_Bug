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
   Compiling regex v1.3.9
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 10.06s
tidy check
tidy error: following path contains more than 2830 entries, you should move the test to some relevant subdirectory (current: 2830): /checkout/src/test/ui/issues
Found 435 error codes
Found 0 error codes with no tests
Done!
some tidy checks failed
