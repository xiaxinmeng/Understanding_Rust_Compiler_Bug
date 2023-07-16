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
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 7.48s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: following path contains more than 968 entries, you should move the test to some relevant subdirectory (current: 969): /checkout/src/test/ui
* 630 error codes
* highest error code: E0788
Found 505 error codes
Found 0 error(s) in error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/src/test/ui/deref-patterns/mir.rs: missing trailing newline
tidy error: /checkout/src/test/ui/deref-patterns/basic.rs: missing trailing newline
some tidy checks failed
Build completed unsuccessfully in 0:00:10
