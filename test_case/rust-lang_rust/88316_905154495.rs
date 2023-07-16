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
    Finished release [optimized] target(s) in 10.60s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: /checkout/src/test/debuginfo/recursive-struct.rs:201: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/boxed-struct.rs:54: line longer than 100 chars
tidy error: /checkout/src/test/debuginfo/boxed-struct.rs:56: line longer than 100 chars
* highest error code: E0784
tidy error: /checkout/src/test/ui/borrowck/borrow-tuple-fields.rs: leading newline
Found 500 error codes
Found 0 error codes with no tests
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/test/ui/ufcs/ufcs-explicit-self-bad.rs: leading newline
* 347 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:13
