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
    Finished release [optimized] target(s) in 10.69s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: following path contains more than 2488 entries, you should move the test to some relevant subdirectory (current: 2490): /checkout/src/test/ui/issues
* highest error code: E0785
Found 501 error codes
Found 0 error codes with no tests
Done!
Done!
* 353 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:13
