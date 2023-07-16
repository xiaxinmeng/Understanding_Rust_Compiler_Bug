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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 627 error codes
* highest error code: E0785
tidy error: /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:117: line longer than 100 chars
tidy error: /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:148: line longer than 100 chars
tidy error: /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:190: line longer than 100 chars
Found 0 error codes with no tests
Done!
* 349 features
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
