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
    Finished release [optimized] target(s) in 10.19s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: duplicate error code: 726
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes.rs:427: E0726: include_str!("./error_codes/E0726.md"),
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes.rs:643:     E0726, // non-explicit (not `'_`) elided lifetime in unsupported position
Found 0 error codes with no tests
Done!
* 351 features
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
