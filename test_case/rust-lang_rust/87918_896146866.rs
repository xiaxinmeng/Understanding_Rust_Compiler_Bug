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
* 626 error codes
* highest error code: E0784
tidy error: /checkout/compiler/rustc_llvm/llvm-wrapper/PassWrapper.cpp:807: line longer than 100 chars
tidy error: /checkout/compiler/rustc_llvm/llvm-wrapper/PassWrapper.cpp:810: line longer than 100 chars
tidy error: /checkout/compiler/rustc_llvm/llvm-wrapper/PassWrapper.cpp:812: line longer than 100 chars
tidy error: /checkout/compiler/rustc_llvm/llvm-wrapper/PassWrapper.cpp:814: line longer than 100 chars
Found 0 error codes with no tests
Done!
* 346 features
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
