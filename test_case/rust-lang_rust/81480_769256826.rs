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
    Finished release [optimized] target(s) in 8.01s
tidy check
tidy error: following path contains more than 2669 entries, you should move the test to some relevant subdirectory (current: 2671): /checkout/src/test/ui/issues
Found 435 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/test/ui/issues/issue-81220.rs:11: trailing whitespace
tidy error: /checkout/src/test/ui/issues/issue-81220.rs:11: trailing whitespace
tidy error: /checkout/src/test/ui/issues/issue-81220.rs:13: trailing whitespace
tidy error: /checkout/src/test/ui/issues/issue-81220.rs:17: trailing whitespace
tidy error: /checkout/src/test/ui/issues/issue-81220.rs:24: trailing whitespace
tidy error: /checkout/src/test/ui/issues/issue-81220.rs: too many trailing newlines (2)
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

