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
    Finished release [optimized] target(s) in 9.07s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: /checkout/src/librustdoc/clean/types.rs:2253: trailing whitespace
tidy error: /checkout/src/librustdoc/clean/types.rs:2255: trailing whitespace
tidy error: /checkout/src/librustdoc/clean/types.rs:2258: trailing whitespace
tidy error: /checkout/src/librustdoc/clean/types.rs:2261: trailing whitespace
* highest error code: E0785
Found 501 error codes
Found 0 error codes with no tests
Done!
Done!
* 354 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:11
